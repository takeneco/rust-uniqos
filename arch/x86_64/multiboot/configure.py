
def config(x):
    mb_rlib = x.outroot('i686-uniqos', x.buildmode(), 'multiboot')

    mb_triple = x.srcpath('i686-uniqos.json')

    mb_mapfile = x.outroot('i686-uniqos', x.buildmode(), 'multiboot.map'
                 ) if x.opt('MAPFILE') else None

    mb_features = []
    if x.opt('boot_multiboot2'): mb_features.append('multiboot/boot_multiboot2')

    x.build_cargo(mb_rlib, 'xbuild',
        pkg='multiboot',
        triple=mb_triple,
        ldscript=x.srcpath('multiboot.ld'),
        rustflags='-Clink-arg=-nostdlib',
        features=mb_features,
        mapfile=mb_mapfile
        )

