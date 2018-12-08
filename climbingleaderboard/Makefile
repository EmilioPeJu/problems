
run:
	python climbingleaderboard.py

test:
	./checkinputs.sh

remote-debug: 
	gdbserver :1234 python climbingleaderboard.py < in_test

all: run
