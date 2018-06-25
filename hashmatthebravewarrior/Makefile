CC=g++
FLAGS=-std=c++11 -ggdb -lm -lcrypt -pipe -ansi -Wall

.DEFAULT_GOAL := all

hashmatthebravewarrior.out: hashmatthebravewarrior.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o hashmatthebravewarrior.out

run: hashmatthebravewarrior.out
	./hashmatthebravewarrior.out

test: hashmatthebravewarrior.out
	./checkinputs.sh hashmatthebravewarrior.out

debug: hashmatthebravewarrior.out
	gdb -x debug_cmd  hashmatthebravewarrior.out

all: hashmatthebravewarrior.out
