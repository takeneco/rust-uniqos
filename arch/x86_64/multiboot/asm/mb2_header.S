/// @brief  Multiboot2 header.
//
/// This code should to be placement to head of kernel image.

.section .mb

// Multiboot2 header

#define ASM_FILE
#include "multiboot2.h"
#undef ASM_FILE

    .align MULTIBOOT_HEADER_ALIGN

    // magic field
mbh:
    .long MULTIBOOT2_HEADER_MAGIC
    .long MULTIBOOT_ARCHITECTURE_I386
    .long mbh_end - mbh
    .long -(MULTIBOOT2_HEADER_MAGIC + MULTIBOOT_ARCHITECTURE_I386 + (mbh_end - mbh))

    // infomation request
inforeq:
    .word MULTIBOOT_HEADER_TAG_INFORMATION_REQUEST, 0
    .long inforeq_end - inforeq
    .long MULTIBOOT_TAG_TYPE_MMAP
    .long MULTIBOOT_TAG_TYPE_FRAMEBUFFER
inforeq_end:

    // end of tags
    .word MULTIBOOT_HEADER_TAG_END, 0
    .long 8
mbh_end:
