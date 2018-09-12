CC=g++
FLAGS=-std=c++17 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

radiostation.out: radiostation.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o radiostation.out

run: radiostation.out
	./radiostation.out

test: radiostation.out
	./checkinputs.sh

debug: radiostation.out
	gdb -x debug_cmd  radiostation.out

all: radiostation.out
