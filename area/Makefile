CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

area.out: area.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o area.out

run: area.out
	./area.out

test: area.out
	./checkinputs.sh

debug: area.out
	gdb -x debug_cmd  area.out

all: area.out
