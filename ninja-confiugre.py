#!/usr/bin/env python3
# Script that generates build scripts for ninja.

import os
import sys

sourcedir = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, os.path.join(sourcedir, 'build'))
import ninjaconf

DEST = 'target'

x = ninjaconf.context(__file__, 'target', 'target/ninja')

x.variable('builddir', DEST)
x.variable('CARGO', os.environ.get('CARGO', 'cargo'))
x.variable('LD', os.environ.get('LD', 'ld'))


x.rule('link', '$LD $ldflags -o $out $in $libs')

x.rule('qemu', 'qemu-system-x86_64 $opts')

# Build util

x.recurse('util')

# Build task for multiboot kernel

if x.opt('boot_multiboot2'):
    mb_kernel = x.outroot('i686-uniqos', x.buildmode(), 'multiboot')
    x.recurse('arch/x86_64/multiboot')
    x.rule('mb_iso',
        'GRUB2_MOD_PATH={} '
	'GRUB2_MKIMAGE={} '
	'MBKERNEL=$kernel '
	'build/iso.sh'.format(
        x.opt('GRUB2_MOD_PATH'),
        x.opt('GRUB2_MKIMAGE')))
    x.build('target/uniqos.iso', 'mb_iso',
        implicit = mb_kernel,
        variables = {'kernel': mb_kernel}
    )

x.default(x.opt('DEFAULT_TARGET'))

x.recurse('arch/x86_64')

# Build task for buutloader and bootimage module kernel

if x.opt('BOOTIMAGE'):
    x.rule('bootimage', 'bootimage build $opts')

    bootimage = x.outroot(
        'x86_64-uniqos', x.buildmode(), 'bootimage-uniqos.bin')

    bootimage_input = '_bootimage-uniqos.bin'
    x.build(bootimage_input, 'phony')

    x.build(bootimage, 'bootimage',
        inputs = bootimage_input,
        implicit = x.srcpath('arch/x86_64/x86_64-uniqos.json'),
        variables = {'opts': '--target arch/x86_64/x86_64-uniqos.json'}
    )
    x.default(bootimage)

# Qemu boot test

x.build('run', 'qemu',
    variables = {'opts' : '-drive format=raw,file=target/x86_64-uniqos/debug/bootimage-uniqos.bin'})
