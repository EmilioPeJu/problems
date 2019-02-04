CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

nicegarland.out: nicegarland.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o nicegarland.out

run: nicegarland.out
	./nicegarland.out

test: nicegarland.out
	./checkinputs.sh

debug: nicegarland.out
	gdb -x debug_cmd  nicegarland.out

remote-debug: nicegarland.out
	gdbserver :1234 nicegarland.out < in1

all: nicegarland.out
