CC=g++
FLAGS=-std=c++11 -ggdb -lm -lcrypt -pipe -ansi -Wall

.DEFAULT_GOAL := all

3n1.out: 3n1.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o 3n1.out

run: 3n1.out
	./3n1.out

test: 3n1.out
	./checkinputs.sh 3n1.out

debug: 3n1.out
	gdb -x debug_cmd  3n1.out

all: 3n1.out
