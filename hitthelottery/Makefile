CC="g++"
FLAGS="-ggdb"

.DEFAULT_GOAL := all

main.out: main.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o main.out

run: main.out
	./main.out

all: main
