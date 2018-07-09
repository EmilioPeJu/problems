CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

weddingshopping.out: weddingshopping.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o weddingshopping.out

run: weddingshopping.out
	./weddingshopping.out

test: weddingshopping.out
	./checkinputs.sh

debug: weddingshopping.out
	gdb -x debug_cmd  weddingshopping.out

all: weddingshopping.out
