# Makefile
CC = gcc
CFLAGS = -std=gnu99 -ffreestanding -O2 -Wall -Wextra
AS = nasm
ASFLAGS = -f elf32
LDFLAGS = -ffreestanding -O2 -nostdlib -lgcc

KERNEL = kernel.bin
ISO = os.iso

C_SOURCES = $(wildcard *.c drivers/*.c lib/*.c)
C_OBJECTS = $(C_SOURCES:.c=.o)
ASM_SOURCES = boot.s
ASM_OBJECTS = $(ASM_SOURCES:.s=.o)

all: $(ISO)

$(ISO): $(KERNEL)
	mkdir -p isodir/boot/grub
	cp $(KERNEL) isodir/boot/kernel.bin
	cp grub.cfg isodir/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) isodir

$(KERNEL): $(ASM_OBJECTS) $(C_OBJECTS)
	$(CC) -T linker.ld -o $@ $(LDFLAGS) $^

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

%.o: %.s
	$(AS) $(ASFLAGS) $< -o $@

clean:
	rm -f *.o drivers/*.o lib/*.o $(KERNEL) $(ISO)
	rm -rf isodir

run: $(ISO)
	qemu-system-i386 -cdrom $(ISO)

debug: $(ISO)
	qemu-system-i386 -cdrom $(ISO) -s -S

.PHONY: all clean run debug