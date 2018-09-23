CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

insearchofeasy.out: insearchofeasy.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o insearchofeasy.out

run: insearchofeasy.out
	./insearchofeasy.out

test: insearchofeasy.out
	./checkinputs.sh

debug: insearchofeasy.out
	gdb -x debug_cmd  insearchofeasy.out

remote-debug: insearchofeasy.out
	gdbserver :1234 insearchofeasy.out < in_test

all: insearchofeasy.out
