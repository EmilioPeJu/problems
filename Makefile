CC=g++
CFLAGS=-std=c++17 -ggdb -O0 -pipe -Wall
CLIBS=-lm -lcrypt

CSRC=$(wildcard *.cpp)
COBJ=$(CSRC:.cpp=.o)
CBIN=$(CSRC:.cpp=.out)

RUST=rustc
RSRC=$(wildcard *.rs)
RBIN=$(RSRC:.rs=.out)

.DEFAULT_GOAL := all

all:: $(COBJ) $(CBIN)

all:: $(RBIN)

%.out:: %.o
	$(CC) $(CLIBS) -o $@ $^

%.out:: %.rs
	$(RUST) -o $@ $^

%.o: %.cpp
	$(CC) $(CFLAGS) -c $<

clean:
	rm -rf $(COBJ) $(CBIN) $(RBIN)

test:
	./checkinputs.sh

