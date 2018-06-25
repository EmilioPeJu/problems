CC=g++
FLAGS=-std=c++11 -ggdb -lm -lcrypt -pipe -ansi -Wall

.DEFAULT_GOAL := all

minesweeper.out: minesweeper.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o minesweeper.out

run: minesweeper.out
	./minesweeper.out

test: minesweeper.out
	./checkinputs.sh minesweeper.out

debug: minesweeper.out
	gdb -x debug_cmd  minesweeper.out

all: minesweeper.out
