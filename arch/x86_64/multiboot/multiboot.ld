OUTPUT_FORMAT(elf32-i386)
OUTPUT_ARCH(i386)
ENTRY(_start)

SECTIONS {
    . = 0x200000 + SIZEOF_HEADERS;
    /*.mb : {
        KEEP(*(.mb))
    }*/
    .text : {
        KEEP(*(.mb))
        KEEP(*(.start))
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.data .data.*)
    }
    .bss : {
        *(.bss .bss.*)
        stack_start = ALIGN(4);
        . = . + 0x1000;
        stack_end = .;
    }
}
