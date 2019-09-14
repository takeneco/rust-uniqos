# default build options

option = {

# Build mode: Release: 'release', Debug: 'debug'
'BUILDMODE' : 'release',

# default target path for ninja-build
'DEFAULT_TARGET' : 'target/uniqos.iso',

# GRUB module path
'GRUB2_MOD_PATH' : '/usr/lib/grub/i386-pc',

# grub2-mkimage command
'GRUB2_MKIMAGE' : 'grub2-mkimage',

'MAPFILE' : False,

# Multiboot2 kernel: True: enable, False: disable
'boot_multiboot2' : True,

# Use bootloader adn bootimage module: True: enable, False: disable
'BOOTIMAGE' : False,

}
