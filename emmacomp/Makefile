CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

emmacomp.out: emmacomp.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o emmacomp.out

run: emmacomp.out
	./emmacomp.out

test: emmacomp.out
	./checkinputs.sh

debug: emmacomp.out
	gdb -x debug_cmd  emmacomp.out

all: emmacomp.out
