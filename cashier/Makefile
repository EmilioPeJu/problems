CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

cashier.out: cashier.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o cashier.out

run: cashier.out
	./cashier.out

test: cashier.out
	./checkinputs.sh

debug: cashier.out
	gdb -x debug_cmd  cashier.out

remote-debug: cashier.out
	gdbserver :1234 cashier.out < in_test

all: cashier.out
