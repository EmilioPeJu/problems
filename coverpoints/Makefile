CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

coverpoints.out: coverpoints.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o coverpoints.out

run: coverpoints.out
	./coverpoints.out

test: coverpoints.out
	./checkinputs.sh

debug: coverpoints.out
	gdb -x debug_cmd  coverpoints.out

all: coverpoints.out
