CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

stages.out: stages.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o stages.out

run: stages.out
	./stages.out

test: stages.out
	./checkinputs.sh

debug: stages.out
	gdb -x debug_cmd  stages.out

remote-debug: stages.out
	gdbserver :1234 stages.out < in_test

all: stages.out
