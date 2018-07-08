CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

jackpot.out: jackpot.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o jackpot.out

run: jackpot.out
	./jackpot.out

test: jackpot.out
	./checkinputs.sh

debug: jackpot.out
	gdb -x debug_cmd  jackpot.out

all: jackpot.out
