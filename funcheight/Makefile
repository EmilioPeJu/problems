CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

funcheight.out: funcheight.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o funcheight.out

run: funcheight.out
	./funcheight.out

test: funcheight.out
	./checkinputs.sh

debug: funcheight.out
	gdb -x debug_cmd  funcheight.out

all: funcheight.out
