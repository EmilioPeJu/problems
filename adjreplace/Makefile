CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

adjreplace.out: adjreplace.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o adjreplace.out

run: adjreplace.out
	./adjreplace.out

test: adjreplace.out
	./checkinputs.sh

debug: adjreplace.out
	gdb -x debug_cmd  adjreplace.out

all: adjreplace.out
