CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

badge.out: badge.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o badge.out

run: badge.out
	./badge.out

test: badge.out
	./checkinputs.sh

debug: badge.out
	gdb -x debug_cmd  badge.out

all: badge.out
