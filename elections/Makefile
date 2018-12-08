CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

elections.out: elections.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o elections.out

run: elections.out
	./elections.out

test: elections.out
	./checkinputs.sh

debug: elections.out
	gdb -x debug_cmd  elections.out

remote-debug: elections.out
	gdbserver :1234 elections.out < in1

all: elections.out
