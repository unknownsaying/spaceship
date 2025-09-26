// drivers/keyboard.c
#include "keyboard.h"
#include "screen.h"

#define KEYBOARD_DATA_PORT 0x60
#define KEYBOARD_STATUS_PORT 0x64

static char key_buffer[256];
static uint32 key_buffer_start = 0;
static uint32 key_buffer_end = 0;

// US QWERTY keyboard scancode map
static const char scancode_to_ascii[128] = {
    0,  27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0, 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`', 
    0, '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0,
    '*', 0, ' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, '-',
    0, 0, 0, '+', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

void keyboard_init(void) {
    // Enable keyboard interrupts
    outb(0x21, 0xFD); // Enable IRQ1 (keyboard)
}

void keyboard_handler(void) {
    uint8 scancode = inb(KEYBOARD_DATA_PORT);
    
    if (scancode & 0x80) {
        // Key release - ignore for now
    } else {
        char key = scancode_to_ascii[scancode];
        if (key != 0) {
            key_buffer[key_buffer_end] = key;
            key_buffer_end = (key_buffer_end + 1) % sizeof(key_buffer);
        }
    }
    
    // Acknowledge interrupt
    outb(0x20, 0x20);
}

bool keyboard_has_key(void) {
    return key_buffer_start != key_buffer_end;
}

char keyboard_get_key(void) {
    while (!keyboard_has_key()) {
        // Wait for key
        asm volatile("hlt");
    }
    
    char key = key_buffer[key_buffer_start];
    key_buffer_start = (key_buffer_start + 1) % sizeof(key_buffer);
    return key;
}