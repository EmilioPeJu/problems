CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

vovaandtrain.out: vovaandtrain.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o vovaandtrain.out

run: vovaandtrain.out
	./vovaandtrain.out

test: vovaandtrain.out
	./checkinputs.sh

debug: vovaandtrain.out
	gdb -x debug_cmd  vovaandtrain.out

remote-debug: vovaandtrain.out
	gdbserver :1234 vovaandtrain.out < in_test

all: vovaandtrain.out
