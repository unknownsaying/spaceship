// include/keyboard.h
#ifndef KEYBOARD_H
#define KEYBOARD_H

#include "types.h"

void keyboard_init(void);
void keyboard_handler(void);
bool keyboard_has_key(void);
char keyboard_get_key(void);

#endif