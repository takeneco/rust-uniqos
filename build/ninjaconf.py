
import os
import zlib

class ninja_context:
    def __init__(self, root, srcdir, outdir, ninja_outdir, option=None):
        self._root = root
        self._srcdir = srcdir
        self._outdir = outdir
        self._ninja_outdir = ninja_outdir
        self._option = option

    def __del__(self):
        if hasattr(self, '_ninja'):
            self._ninja.close()

    def srcpath(self, *relpaths):
        return os.path.normpath(os.path.join(self._srcdir, *relpaths))

    def outpath(self, *relpaths):
        return os.path.normpath(
               os.path.join(self._outdir, self._srcdir, *relpaths))

    def outroot(self, *relpaths):
        return os.path.normpath(os.path.join(self._outdir, *relpaths))

    def recurse(self, relpath):
        srcdir = os.path.join(self._srcdir, relpath)
        ctx = ninja_context(
            self._root, srcdir, self._outdir, self._ninja_outdir, self._option)
        script_path = os.path.join(self._root, ctx._srcdir, 'configure.py')

        with open(script_path, 'r') as f:
            code = compile(f.read(), script_path, 'exec')
            code_dic = {}
            exec(code, code_dic)
            code_dic['config'](ctx)

            if hasattr(ctx, '_ninjapath'):
                ninja = self._open()
                ninja.write('include {}\n'.format(ctx._ninjapath))

    def variable(self, key, value, indent=0):
        ninja = self._open()
        ninja.write('{}{} = {}\n'.format('  ' * indent, key, value))

    def rule(self, name, command, description=None, depfile=None,
             generator=False, pool=None, restat=False, rspfile=None,
             rspfile_content=None, deps=None):
        ninja = self._open()
        ninja.write('rule {}\n'.format(name))
        self.variable('command', command, indent=1)
        if description:
            self.variable('description', description, indent=1)
        if depfile:
            self.variable('depfile', depfile, indent=1)
        if generator:
            self.variable('generator', '1', indent=1)
        if pool:
            self.variable('pool', pool, indent=1)
        if restat:
            self.variable('restat', '1', indent=1)
        if rspfile:
            self.variable('rspfile', rspfile, indent=1)
        if rspfile_content:
            self.variable('rspfile_content', rspfile_content, indent=1)
        if deps:
            self.variable('deps', deps, indent=1)

    def build(self, outputs, rule, inputs=None, implicit=None, order_only=None,
              variables=None, implicit_outputs=None, pool=None):
        outputs = to_str(outputs)
        if implicit_outputs:
            outputs = outputs + ' | ' + to_str(implicit_outputs)

        inputs = to_str(inputs)
        if implicit:
            inputs = inputs + ' | ' + to_str(implicit)
        if order_only:
            inputs = inputs + ' || ' + to_str(order_only)

        ninja = self._open()
        ninja.write('build {} : {} {}\n'.format(outputs, rule, inputs))
        if pool:
            self.variable('pool', pool, indent=1)
        if variables:
            for key, val in variables.items():
                self.variable(key, val, indent=1)

    def default(self, target):
        ninja = self._open()
        ninja.write('default {}\n'.format(target))

    def opt(self, key):
        return self._option[key]

    def _open(self):
        ninja = getattr(self, '_ninja', None)
        if not ninja:
            if self._srcdir:
                name = self._srcdir.replace('/', '_') + '.ninja'
            else:
                name = 'root.ninja'
            self._ninjapath = os.path.join(self._ninja_outdir, name)
            path = os.path.join(self._root, self._ninjapath)
            print(path)
            self._ninja = open(path, 'w')
        return self._ninja

    def _load_option(self):
        import default_option
        self._option = default_option.option
        opt_path = os.path.join(self._root, 'option')
        try:
            with open(opt_path) as f:
                code = compile(f.read(), opt_path, 'exec')
                exec(code, self._option)
        except FileNotFoundError as e:
            # Apply defualt configuration.
            pass

    def build_cargo(self, output, subcommand, pkg=None,
                    dir='.', triple=None, ldscript=None, rustflags=None,
                    features=[],mapfile=None):
        phony_src = '{0}-{1:08x}'.format(os.path.basename(output),
                                     zlib.adler32(output.encode()) & 0xffffffff)
        implicits = []
        opts = []
        _rustflags = []
        if pkg:
            opts.append('-p' + pkg)
        if triple:
            opts.append('--target ' + triple)
        if features:
            opts.extend(['--features', '\'' + ' '.join(features) + '\''])
        if ldscript:
            _rustflags.append('-Clink-arg=-T' + ldscript)
            implicits.append(ldscript)
        if rustflags:
            _rustflags.append(rustflags)
        if mapfile:
            #_rustflags.append('-Clink-arg=-Wl,-Map,'+mapfile) 
            _rustflags.append('-Clink-arg=-Map,'+mapfile) 
        if self._option['BUILDMODE'] == 'release':
            opts.append('--release')

        self.build(phony_src, 'phony')
        self.build(output, 'cargo', inputs = phony_src, implicit = implicits,
            variables = {
                'dir' : dir,
                'sub' : subcommand,
                'opts' : to_str(opts),
                'rustflags' : to_str(_rustflags),
            }
        )

    def buildmode(self):
        return self._option['BUILDMODE']

def _default_rules(x):
    x.rule('cargo', 
        # 'cd $dir && RUSTFLAGS="$rustflags" $CARGO -vv $sub $opts',
        'cd $dir && RUSTFLAGS="$rustflags" $CARGO -q $sub $opts',
        restat=True)

def to_str(val, sep=' '):
    if not val:
        val = ''
    if isinstance(val, list) or isinstance(val, tuple):
        val = sep.join(val)
    return val

def context(caller_path, output_path, ninja_output_path=None):
    if not ninja_output_path:
        ninja_output_path = output_path
    os.makedirs(output_path, exist_ok=True)
    os.makedirs(ninja_output_path, exist_ok=True)
    x = ninja_context(
        os.path.normpath(os.path.dirname(caller_path)),
        '',
        output_path,
        ninja_output_path)
    x._load_option()
    _default_rules(x)
    return x

