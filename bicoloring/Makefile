CC=g++
FLAGS=-std=c++11 -ggdb -lm -lcrypt -pipe -ansi -Wall

.DEFAULT_GOAL := all

bicoloring.out: bicoloring.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o bicoloring.out

run: bicoloring.out
	./bicoloring.out

test: bicoloring.out
	./checkinputs.sh

debug: bicoloring.out
	gdb -x debug_cmd  bicoloring.out

all: bicoloring.out
