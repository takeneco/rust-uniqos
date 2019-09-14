#!/bin/sh

if [ ! -d "$GRUB2_MOD_PATH" ]; then
    echo 'GRUB2_MOD_PATH is not given'
    exit 1
fi

if ! which "$GRUB2_MKIMAGE" > /dev/null; then
    echo 'GRUB2_MKIMAGE is not given'
    exit 1
fi

if ! file "$MBKERNEL" > /dev/null; then
    echo 'MBKERNEL is not given'
    exit 1
fi

#KERNEL=target/i686-uniqos/debug/multiboot

rm -f target/iso/kernel
mkdir -p target/iso/boot/grub
cp $MBKERNEL target/iso/kernel

cat > target/iso/boot/grub/grub.cfg <<CFG
default='Uniqos'
timeout=10
menuentry 'Uniqos' {
set root=(cd)
multiboot2 (cd)/kernel root=cd0
#multiboot (cd)/kernel
#module2 --nounzip (cd)/test
}
CFG

#grub2-mkrescue -o target/uniqos.iso target/iso
#exit

cat > target/load_cfg <<CFG
insmod part_msdos
CFG

ELTORITO=target/iso/boot/grub/i386-pc/eltorito.img
if [ ! -f $ELTORITO ]; then
  rm -fr target/iso/boot/grub/i386-pc
  mkdir -p target/iso/boot/grub/i386-pc
  cp $GRUB2_MOD_PATH/*.mod $GRUB2_MOD_PATH/*.lst target/iso/boot/grub/i386-pc
  $GRUB2_MKIMAGE \
   -O i386-pc \
   -d $GRUB2_MOD_PATH \
   -o target/core.img \
   -c target/load_cfg \
   --prefix=/boot/grub \
   iso9660 biosdisk

  cat $GRUB2_MOD_PATH/cdboot.img target/core.img \
   > target/iso/boot/grub/i386-pc/eltorito.img
fi

mkisofs \
 -o target/uniqos.iso \
 -b boot/grub/i386-pc/eltorito.img \
 -c boot/boot.cat \
 -no-emul-boot \
 -boot-load-size 4 \
 -boot-info-table \
 -R -J target/iso/

