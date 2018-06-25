CC=g++
FLAGS=-std=c++11 -ggdb

.DEFAULT_GOAL := all

backtohighschoolphysics.out: backtohighschoolphysics.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o backtohighschoolphysics.out

run: backtohighschoolphysics.out
	./backtohighschoolphysics.out

test: backtohighschoolphysics.out
	./checkinputs.sh backtohighschoolphysics.out

all: backtohighschoolphysics.out
