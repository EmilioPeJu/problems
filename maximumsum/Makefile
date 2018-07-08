CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

maximumsum.out: maximumsum.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o maximumsum.out

run: maximumsum.out
	./maximumsum.out

test: maximumsum.out
	./checkinputs.sh

debug: maximumsum.out
	gdb -x debug_cmd  maximumsum.out

all: maximumsum.out
