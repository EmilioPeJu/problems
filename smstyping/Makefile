CC=g++
FLAGS=-std=c++11 -ggdb -lm -lcrypt -pipe -ansi -Wall

.DEFAULT_GOAL := all

smstyping.out: smstyping.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o smstyping.out

run: smstyping.out
	./smstyping.out

test: smstyping.out
	./checkinputs.sh

debug: smstyping.out
	gdb -x debug_cmd  smstyping.out

all: smstyping.out
