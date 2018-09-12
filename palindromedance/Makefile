CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

palindromedance.out: palindromedance.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o palindromedance.out

run: palindromedance.out
	./palindromedance.out

test: palindromedance.out
	./checkinputs.sh

debug: palindromedance.out
	gdb -x debug_cmd  palindromedance.out

all: palindromedance.out
