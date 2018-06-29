CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

hotelbooking.out: hotelbooking.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o hotelbooking.out

run: hotelbooking.out
	./hotelbooking.out

test: hotelbooking.out
	./checkinputs.sh

debug: hotelbooking.out
	gdb -x debug_cmd  hotelbooking.out

all: hotelbooking.out
