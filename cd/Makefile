CC=g++
FLAGS=-std=c++11 -ggdb -lm -lcrypt -pipe -ansi -Wall

.DEFAULT_GOAL := all

cd.out: cd.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o cd.out

run: cd.out
	./cd.out

test: cd.out
	./checkinputs.sh cd.out

debug: cd.out
	gdb -x debug_cmd cd.out

all: cd.out
