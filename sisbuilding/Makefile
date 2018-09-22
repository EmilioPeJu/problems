CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

sisbuilding.out: sisbuilding.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o sisbuilding.out

run: sisbuilding.out
	./sisbuilding.out

test: sisbuilding.out
	./checkinputs.sh

debug: sisbuilding.out
	gdb -x debug_cmd  sisbuilding.out

all: sisbuilding.out
