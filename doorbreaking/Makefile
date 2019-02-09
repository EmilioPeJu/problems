CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

doorbreaking.out: doorbreaking.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o doorbreaking.out

run: doorbreaking.out
	./doorbreaking.out

test: doorbreaking.out
	./checkinputs.sh

debug: doorbreaking.out
	gdb -x debug_cmd  doorbreaking.out

remote-debug: doorbreaking.out
	gdbserver :1234 doorbreaking.out < in1

all: doorbreaking.out
