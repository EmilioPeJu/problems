CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

forgery.out: forgery.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o forgery.out

run: forgery.out
	./forgery.out

test: forgery.out
	./checkinputs.sh

debug: forgery.out
	gdb -x debug_cmd  forgery.out

remote-debug: forgery.out
	gdbserver :1234 forgery.out < in_test

all: forgery.out
