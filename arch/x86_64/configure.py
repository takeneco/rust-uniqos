
def config(x):
    if x.opt('boot_multiboot2'):
        mb_kernel = x.outroot('i686-uniqos', x.buildmode(), 'multiboot')
    #x86_64_rlib = x.outroot('x86_64-uniqos/debug/libx86_64.rlib')

    #x.build('_libx86_64.rlib', 'phony')
    #x.build(x86_64_rlib, 'cargo',
    #    inputs='_libx86_64.rlib',
    #    implicit=x.srcpath('x86_64-uniqos.json'),
    #    variables={
    #        'dir' : x.srcpath(),
    #        'sub': 'xbuild',
    #        'opts': '--target x86_64-uniqos.json',
    #    }
    #)

