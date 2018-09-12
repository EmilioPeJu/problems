CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

findsquare.out: findsquare.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o findsquare.out

run: findsquare.out
	./findsquare.out

test: findsquare.out
	./checkinputs.sh

debug: findsquare.out
	gdb -x debug_cmd  findsquare.out

all: findsquare.out
