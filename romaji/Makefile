CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

romaji.out: romaji.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o romaji.out

run: romaji.out
	./romaji.out

test: romaji.out
	./checkinputs.sh

debug: romaji.out
	gdb -x debug_cmd  romaji.out

all: romaji.out
