CC=g++
FLAGS=-std=c++11 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

potentiometers.out: potentiometers.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o potentiometers.out

run: potentiometers.out
	./potentiometers.out

test: potentiometers.out
	./checkinputs.sh

debug: potentiometers.out
	gdb -x debug_cmd  potentiometers.out

remote-debug: potentiometers.out
	gdbserver :1234 potentiometers.out < in_test

all: potentiometers.out
