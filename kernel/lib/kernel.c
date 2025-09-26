// kernel.c
#include "kernel.h"
#include "screen.h"
#include "keyboard.h"
#include "timer.h"
#include "string.h"
#include "memory.h"

// Global variables
uint32* page_directory = 0;
uint32 page_table[1024] __attribute__((aligned(4096)));

// Kernel entry point
void kernel_main(void) {
    // Initialize screen first
    screen_clear();
    screen_print("Simple Kernel Booted Successfully!\n");
    screen_print("Initializing system...\n");
    
    // Initialize memory management
    memory_init();
    screen_print("Memory manager initialized\n");
    
    // Initialize paging
    init_paging();
    screen_print("Paging initialized\n");
    
    // Initialize interrupt descriptor table
    init_idt();
    screen_print("Interrupts initialized\n");
    
    // Initialize hardware
    timer_init(100); // 100 Hz
    keyboard_init();
    screen_print("Hardware initialized\n");
    
    // Welcome message
    screen_print("\n=== Simple Kernel v0.1 ===\n");
    screen_print("Commands: help, clear, echo, meminfo\n");
    screen_print("Kernel ready. Type 'help' for commands.\n");
    screen_print("> ");
    
    // Main kernel loop
    kernel_loop();
}

// Simple kernel shell
void kernel_loop(void) {
    char input[256];
    uint32 input_pos = 0;
    
    while (1) {
        // Handle keyboard input
        if (keyboard_has_key()) {
            char key = keyboard_get_key();
            
            if (key == '\n') {
                input[input_pos] = '\0';
                execute_command(input);
                input_pos = 0;
                screen_print("> ");
            } else if (key == '\b') {
                if (input_pos > 0) {
                    input_pos--;
                    screen_backspace();
                }
            } else if (input_pos < sizeof(input) - 1) {
                input[input_pos++] = key;
                screen_putchar(key);
            }
        }
    }
}

// Command interpreter
void execute_command(const char* command) {
    screen_print("\n");
    
    if (strcmp(command, "help") == 0) {
        screen_print("Available commands:\n");
        screen_print("  help    - Show this help\n");
        screen_print("  clear   - Clear screen\n");
        screen_print("  echo    - Echo arguments\n");
        screen_print("  meminfo - Show memory information\n");
    }
    else if (strcmp(command, "clear") == 0) {
        screen_clear();
    }
    else if (strncmp(command, "echo ", 5) == 0) {
        screen_print(command + 5);
        screen_print("\n");
    }
    else if (strcmp(command, "meminfo") == 0) {
        memory_info();
    }
    else if (strlen(command) > 0) {
        screen_print("Unknown command: ");
        screen_print(command);
        screen_print("\n");
    }
}

// Panic function for critical errors
void panic(const char* message) {
    screen_print("\n*** KERNEL PANIC ***\n");
    screen_print(message);
    screen_print("\nSystem halted.\n");
    
    asm volatile("cli");
    while (1) {
        asm volatile("hlt");
    }
}

// Basic paging initialization
void init_paging(void) {
    // Identity map first 4MB
    for (int i = 0; i < 1024; i++) {
        page_table[i] = (i * 0x1000) | 3; // Present + Read/Write
    }
    
    page_directory = (uint32*)0x9000;
    page_directory[0] = ((uint32)page_table) | 3;
    
    for (int i = 1; i < 1024; i++) {
        page_directory[i] = 0; // Not present
    }
    
    // Load page directory and enable paging
    asm volatile(
        "mov %0, %%cr3\n"
        "mov %%cr0, %%eax\n"
        "or $0x80000000, %%eax\n"
        "mov %%eax, %%cr0"
        : : "r"(page_directory) : "eax"
    );
}