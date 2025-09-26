// include/timer.h
#ifndef TIMER_H
#define TIMER_H

#include "types.h"

void timer_init(uint32 frequency);
void timer_handler(void);
uint32 timer_get_ticks(void);
void sleep(uint32 milliseconds);

#endif