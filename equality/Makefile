CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

equality.out: equality.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o equality.out

run: equality.out
	./equality.out

test: equality.out
	./checkinputs.sh

debug: equality.out
	gdb -x debug_cmd  equality.out

all: equality.out
