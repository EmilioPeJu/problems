CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

polycarppractice.out: polycarppractice.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o polycarppractice.out

run: polycarppractice.out
	./polycarppractice.out

test: polycarppractice.out
	./checkinputs.sh

debug: polycarppractice.out
	gdb -x debug_cmd  polycarppractice.out

all: polycarppractice.out
