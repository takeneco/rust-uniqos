/// @brief  Kernel entry point.

// Kernel entry point

.section .text

.code32

.globl _start
_start:
    //movb  $'a', 0xb8000
    //movb  $0xf, 0xb8001
    //movb  $'b', 0xb8002
    //movb  $0xf, 0xb8003

    cli

    movl  %eax, %edx

    movl  $stack_end, %esp

    pushl %ebx  // multiboot information
    pushl %edx  // multiboot magic

    call  load
    testl %eax, %eax

//    sti
    jnz   stop

stop:
    hlt
    jmp stop
