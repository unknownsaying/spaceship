; boot.s - Multiboot compliant bootloader
section .multiboot
align 4
    dd 0x1BADB002              ; Magic number
    dd 0x00000003              ; Flags
    dd -(0x1BADB002 + 0x00000003) ; Checksum

section .bss
align 16
stack_bottom:
    resb 16384                 ; 16KB stack
stack_top:

section .text
global _start
extern kernel_main

_start:
    ; Set up stack
    mov esp, stack_top
    
    ; Clear direction flag
    cld
    
    ; Call kernel main
    call kernel_main
    
    ; If kernel returns, halt
    cli
.hang:
    hlt
    jmp .hang