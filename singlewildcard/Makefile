CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

singlewildcard.out: singlewildcard.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o singlewildcard.out

run: singlewildcard.out
	./singlewildcard.out

test: singlewildcard.out
	./checkinputs.sh

debug: singlewildcard.out
	gdb -x debug_cmd  singlewildcard.out

all: singlewildcard.out
