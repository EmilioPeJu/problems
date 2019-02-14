CC=g++
FLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
LIBS=-lm -lcrypt

SRC=$(wildcard *.cpp)
OBJ=$(SRC:.cpp=.o)
BIN=$(SRC:.cpp=)

.DEFAULT_GOAL := all

all: $(OBJ) $(BIN)

%: %.o
	$(CC) $(LIBS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf $(OBJ) $(BIN)

test:
	checkinputs

