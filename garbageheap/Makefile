CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

garbageheap.out: garbageheap.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o garbageheap.out

run: garbageheap.out
	./garbageheap.out

test: garbageheap.out
	./checkinputs.sh

debug: garbageheap.out
	gdb -x debug_cmd  garbageheap.out

all: garbageheap.out
