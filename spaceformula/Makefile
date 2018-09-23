CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

spaceformula.out: spaceformula.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o spaceformula.out

run: spaceformula.out
	./spaceformula.out

test: spaceformula.out
	./checkinputs.sh

debug: spaceformula.out
	gdb -x debug_cmd  spaceformula.out

remote-debug: spaceformula.out
	gdbserver :1234 spaceformula.out < in_test

all: spaceformula.out
