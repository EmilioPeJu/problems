CC=g++
FLAGS=-std=c++11 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

wakingupbrain.out: wakingupbrain.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o wakingupbrain.out

run: wakingupbrain.out
	./wakingupbrain.out

test: wakingupbrain.out
	./checkinputs.sh

debug: wakingupbrain.out
	gdb -x debug_cmd  wakingupbrain.out

remote-debug: wakingupbrain.out
	gdbserver :1234 wakingupbrain.out < in_test

all: wakingupbrain.out
