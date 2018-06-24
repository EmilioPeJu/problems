CC="g++"
FLAGS="-ggdb"

.DEFAULT_GOAL := all

machinedsurface.out: machinedsurface.o
	$(CC) $(FLAGS) -o $@ $^

%.o: %.cpp
	$(CC) $(FLAGS) -c $<

clean:
	rm -rf *.o machinedsurface.out

run: machinedsurface.out
	./machinedsurface.out

test: machinedsurface.out
	./checkinputs.sh machinedsurface.out

all: machinedsurface.out
