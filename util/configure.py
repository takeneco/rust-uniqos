
def config(x):
    rlib = x.outroot('x86_64-uniqos/debug/libutil.rlib')

    x.build_cargo(rlib, 'xbuild',
        pkg='util',
        dir=x.srcpath(),
        triple='x86_64-uniqos.json',
        rustflags='-Clink-arg=-nostdlib')

    x.build_cargo('util-test', 'test', dir=x.srcpath())
