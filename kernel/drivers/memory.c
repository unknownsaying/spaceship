// include/memory.h
#ifndef MEMORY_H
#define MEMORY_H

#include "types.h"

void memory_init(void);
void* kmalloc(size_t size);
void kfree(void* ptr);
void memory_info(void);

#endif