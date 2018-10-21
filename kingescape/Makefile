CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

kingescape.out: kingescape.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o kingescape.out

run: kingescape.out
	./kingescape.out

test: kingescape.out
	./checkinputs.sh

debug: kingescape.out
	gdb -x debug_cmd  kingescape.out

remote-debug: kingescape.out
	gdbserver :1234 kingescape.out < in_test

all: kingescape.out
