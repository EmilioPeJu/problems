CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

flightplanner.out: flightplanner.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o flightplanner.out

run: flightplanner.out
	./flightplanner.out

test: flightplanner.out
	./checkinputs.sh

debug: flightplanner.out
	gdb -x debug_cmd  flightplanner.out

all: flightplanner.out
