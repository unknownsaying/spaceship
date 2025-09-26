// include/kernel.h
#ifndef KERNEL_H
#define KERNEL_H

#include "types.h"

// Kernel main functions
void kernel_main(void);
void panic(const char* message);

// Interrupt handlers
void interrupt_handler(uint32 interrupt_number);
void irq_handler(uint32 irq_number);

// System calls
void syscall_handler(uint32 syscall_number, uint32 arg1, uint32 arg2, uint32 arg3);

#endif