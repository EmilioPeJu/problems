CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

noncopart.out: noncopart.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o noncopart.out

run: noncopart.out
	./noncopart.out

test: noncopart.out
	./checkinputs.sh

debug: noncopart.out
	gdb -x debug_cmd  noncopart.out

all: noncopart.out
