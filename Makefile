CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

%.out: %.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o *.out

test:
	checkinputs

