CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

lunarnewyear.out: lunarnewyear.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o lunarnewyear.out

run: lunarnewyear.out
	./lunarnewyear.out

test: lunarnewyear.out
	./checkinputs.sh

debug: lunarnewyear.out
	gdb -x debug_cmd  lunarnewyear.out

remote-debug: lunarnewyear.out
	gdbserver :1234 lunarnewyear.out < in1

all: lunarnewyear.out
