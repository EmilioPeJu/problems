CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

palindromictwist.out: palindromictwist.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o palindromictwist.out

run: palindromictwist.out
	./palindromictwist.out

test: palindromictwist.out
	./checkinputs.sh

debug: palindromictwist.out
	gdb -x debug_cmd  palindromictwist.out

all: palindromictwist.out
