CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

productrectanglecpp.out: productrectanglecpp.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o productrectanglecpp.out

run: productrectanglecpp.out
	./productrectanglecpp.out

test: productrectanglecpp.out
	./checkinputs.sh

debug: productrectanglecpp.out
	gdb -x debug_cmd  productrectanglecpp.out

remote-debug: productrectanglecpp.out
	gdbserver :1234 productrectanglecpp.out < in1

all: productrectanglecpp.out
