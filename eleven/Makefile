CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

eleven.out: eleven.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o eleven.out

run: eleven.out
	./eleven.out

test: eleven.out
	./checkinputs.sh

debug: eleven.out
	gdb -x debug_cmd  eleven.out

all: eleven.out
