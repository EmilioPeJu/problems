all: challenge1.out challenge2.out challenge3.out

%.out: %.asm
	yasm -f elf64 $< -o $@.o
	ld $@.o -o $@
	chmod +x $@

clean:
	rm *.out *.o

