// drivers/timer.c
#include "timer.h"

#define PIT_FREQUENCY 1193180
#define PIT_COMMAND_PORT 0x43
#define PIT_CHANNEL0_PORT 0x40

static uint32 tick_count = 0;

void timer_init(uint32 frequency) {
    uint32 divisor = PIT_FREQUENCY / frequency;
    
    // Send command byte
    outb(PIT_COMMAND_PORT, 0x36);
    
    // Send divisor low and high bytes
    outb(PIT_CHANNEL0_PORT, divisor & 0xFF);
    outb(PIT_CHANNEL0_PORT, (divisor >> 8) & 0xFF);
}

void timer_handler(void) {
    tick_count++;
    
    // Acknowledge interrupt
    outb(0x20, 0x20);
}

uint32 timer_get_ticks(void) {
    return tick_count;
}

void sleep(uint32 milliseconds) {
    uint32 start = timer_get_ticks();
    uint32 ticks_needed = milliseconds / 10; // 100Hz = 10ms per tick
    
    while (timer_get_ticks() - start < ticks_needed) {
        asm volatile("hlt");
    }
}