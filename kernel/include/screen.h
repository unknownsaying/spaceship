// include/screen.h
#ifndef SCREEN_H
#define SCREEN_H

#include "types.h"

void screen_clear(void);
void screen_putchar(char c);
void screen_print(const char* str);
void screen_backspace(void);
void update_cursor(void);
void outb(uint16 port, uint8 value);
uint8 inb(uint16 port);

#endif