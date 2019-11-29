CC=g++
CFLAGS=-std=c++17 -ggdb -O0 -pipe -Wall -Werror -Wtype-limits
CLIBS=-lm -lcrypt

CSRC=$(wildcard *.cpp)
COBJ=$(CSRC:.cpp=.o)
CBIN=$(CSRC:.cpp=.out)

RUST=rustc
RFLAGS=-g
RSRC=$(wildcard *.rs)
RBIN=$(RSRC:.rs=.out)

.DEFAULT_GOAL := all

all:: $(COBJ) $(CBIN)

all:: $(RBIN)

%.out:: %.o
	$(CC) $(CLIBS) -o $@ $^

%.out:: %.rs
	$(RUST) $(RFLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(CFLAGS) -c $<

clean: clean-tests
	rm -rf $(COBJ) $(CBIN) $(RBIN)

clean-tests:
	rm -rf in?? out?? current??

test:
	./checkinputs.sh

gdb:
	./rungdb.sh
