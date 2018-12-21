all: reverse0.out max1.out rpn2.out tree3.out

%.out: %.asm
	yasm -f elf64 $< -o $@.o
	ld $@.o -o $@
	chmod +x $@

clean:
	rm *.out *.o

