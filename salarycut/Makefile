CC=g++
FLAGS=-std=c++11 -ggdb -pipe -Wall
LIBS=-lm -lcrypt

.DEFAULT_GOAL := all

11727salarycut.out: 11727salarycut.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o 11727salarycut.out

run: 11727salarycut.out
	./11727salarycut.out

test: 11727salarycut.out
	./checkinputs.sh

debug: 11727salarycut.out
	gdb -x debug_cmd  11727salarycut.out

all: 11727salarycut.out
