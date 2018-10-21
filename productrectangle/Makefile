
run:
	python productrectangle.py

test:
	./checkinputs.sh

remote-debug: 
	gdbserver :1234 python productrectangle.py < in_test

all: run
