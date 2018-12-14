all: challenge1 challenge2

%: %.asm
	yasm -f elf64 $< -o $@.o
	ld $@.o -o $@.out
	chmod +x $@.out

clean:
	rm *.out

