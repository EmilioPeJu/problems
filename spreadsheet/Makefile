CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

spreadsheet.out: spreadsheet.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o spreadsheet.out

run: spreadsheet.out
	./spreadsheet.out

test: spreadsheet.out
	./checkinputs.sh

debug: spreadsheet.out
	gdb -x debug_cmd  spreadsheet.out

all: spreadsheet.out
