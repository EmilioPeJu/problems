CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

therank.out: therank.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o therank.out

run: therank.out
	./therank.out

test: therank.out
	./checkinputs.sh

debug: therank.out
	gdb -x debug_cmd  therank.out

all: therank.out
