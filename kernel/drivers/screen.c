// drivers/screen.c
#include "screen.h"
#include "types.h"

#define VIDEO_MEMORY 0xB8000
#define SCREEN_WIDTH 80
#define SCREEN_HEIGHT 25

static uint16* video_memory = (uint16*)VIDEO_MEMORY;
static uint8 cursor_x = 0;
static uint8 cursor_y = 0;

void screen_clear(void) {
    for (int y = 0; y < SCREEN_HEIGHT; y++) {
        for (int x = 0; x < SCREEN_WIDTH; x++) {
            const uint16 blank = ' ' | (0x07 << 8);
            video_memory[y * SCREEN_WIDTH + x] = blank;
        }
    }
    cursor_x = 0;
    cursor_y = 0;
    update_cursor();
}

void screen_putchar(char c) {
    if (c == '\n') {
        cursor_x = 0;
        cursor_y++;
    } else if (c == '\b') {
        if (cursor_x > 0) {
            cursor_x--;
        } else if (cursor_y > 0) {
            cursor_y--;
            cursor_x = SCREEN_WIDTH - 1;
        }
        video_memory[cursor_y * SCREEN_WIDTH + cursor_x] = ' ' | (0x07 << 8);
    } else {
        const uint16 entry = c | (0x07 << 8);
        video_memory[cursor_y * SCREEN_WIDTH + cursor_x] = entry;
        cursor_x++;
    }
    
    if (cursor_x >= SCREEN_WIDTH) {
        cursor_x = 0;
        cursor_y++;
    }
    
    if (cursor_y >= SCREEN_HEIGHT) {
        scroll_screen();
        cursor_y = SCREEN_HEIGHT - 1;
    }
    
    update_cursor();
}

void screen_print(const char* str) {
    for (size_t i = 0; str[i] != '\0'; i++) {
        screen_putchar(str[i]);
    }
}

void scroll_screen(void) {
    // Move all lines up by one
    for (int y = 0; y < SCREEN_HEIGHT - 1; y++) {
        for (int x = 0; x < SCREEN_WIDTH; x++) {
            video_memory[y * SCREEN_WIDTH + x] = video_memory[(y + 1) * SCREEN_WIDTH + x];
        }
    }
    
    // Clear bottom line
    for (int x = 0; x < SCREEN_WIDTH; x++) {
        video_memory[(SCREEN_HEIGHT - 1) * SCREEN_WIDTH + x] = ' ' | (0x07 << 8);
    }
}

void update_cursor(void) {
    uint16 cursor_position = cursor_y * SCREEN_WIDTH + cursor_x;
    
    // CRT control register - cursor location high byte
    outb(0x3D4, 14);
    outb(0x3D5, cursor_position >> 8);
    
    // CRT control register - cursor location low byte
    outb(0x3D4, 15);
    outb(0x3D5, cursor_position);
}

void screen_backspace(void) {
    if (cursor_x > 0) {
        cursor_x--;
    } else if (cursor_y > 0) {
        cursor_y--;
        cursor_x = SCREEN_WIDTH - 1;
    }
    video_memory[cursor_y * SCREEN_WIDTH + cursor_x] = ' ' | (0x07 << 8);
    update_cursor();
}

// Output byte to port
void outb(uint16 port, uint8 value) {
    asm volatile ("outb %1, %0" : : "dN" (port), "a" (value));
}

// Input byte from port
uint8 inb(uint16 port) {
    uint8 ret;
    asm volatile ("inb %1, %0" : "=a" (ret) : "dN" (port));
    return ret;
}