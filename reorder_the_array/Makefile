CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

reorder_the_array.out: reorder_the_array.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o reorder_the_array.out

run: reorder_the_array.out
	./reorder_the_array.out

test: reorder_the_array.out
	./checkinputs.sh

debug: reorder_the_array.out
	gdb -x debug_cmd  reorder_the_array.out

all: reorder_the_array.out
