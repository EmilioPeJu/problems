CC=g++
FLAGS=-std=c++11 -ggdb -lm -lcrypt -pipe -ansi -Wall

.DEFAULT_GOAL := all

horrordash.out: horrordash.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o horrordash.out

run: horrordash.out
	./horrordash.out

test: horrordash.out
	./checkinputs.sh horrordash.out

all: horrordash.out
