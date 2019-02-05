CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

brutality.out: brutality.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o brutality.out

run: brutality.out
	./brutality.out

test: brutality.out
	./checkinputs.sh

debug: brutality.out
	gdb -x debug_cmd  brutality.out

remote-debug: brutality.out
	gdbserver :1234 brutality.out < in1

all: brutality.out
