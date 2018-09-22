CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

littlecloves3.out: littlecloves3.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o littlecloves3.out

run: littlecloves3.out
	./littlecloves3.out

test: littlecloves3.out
	./checkinputs.sh

debug: littlecloves3.out
	gdb -x debug_cmd  littlecloves3.out

all: littlecloves3.out
