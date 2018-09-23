CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

vasyaandcornfield.out: vasyaandcornfield.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o vasyaandcornfield.out

run: vasyaandcornfield.out
	./vasyaandcornfield.out

test: vasyaandcornfield.out
	./checkinputs.sh

debug: vasyaandcornfield.out
	gdb -x debug_cmd  vasyaandcornfield.out

remote-debug: vasyaandcornfield.out
	gdbserver :1234 vasyaandcornfield.out < in_test

all: vasyaandcornfield.out
