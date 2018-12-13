all: challenge1

%: %.asm
	yasm -f elf64 $< -o $@.o
	gcc $@.o -o $@.out
	chmod +x $@.out

clean:
	rm *.out

