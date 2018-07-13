CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

turntherectangles.out: turntherectangles.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o turntherectangles.out

run: turntherectangles.out
	./turntherectangles.out

test: turntherectangles.out
	./checkinputs.sh

debug: turntherectangles.out
	gdb -x debug_cmd  turntherectangles.out

all: turntherectangles.out
