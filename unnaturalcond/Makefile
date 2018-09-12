CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

unnaturalcond.out: unnaturalcond.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o unnaturalcond.out

run: unnaturalcond.out
	./unnaturalcond.out

test: unnaturalcond.out
	./checkinputs.sh

debug: unnaturalcond.out
	gdb -x debug_cmd  unnaturalcond.out

all: unnaturalcond.out
