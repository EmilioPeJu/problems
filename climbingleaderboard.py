#!/usr/bin/env python
import bisect


def get_stdin_from_file():
    import os
    import sys
    TEST_FILE = 'in1'
    if os.environ.get('DEBUG', '0') != '0' \
            and os.access(TEST_FILE, os.R_OK):
        sys.stdin = open(TEST_FILE, 'r')


def main():
    get_stdin_from_file()
    n = int(input())
    scores = list(map(int, input().strip().split()))
    m = int(input())
    alice = list(map(int, input().strip().split()))
    scores = list(set(scores))
    scores.sort()
    for current in alice:
        index =  bisect.bisect(scores, current)
        if  index < len(scores) and scores[index] == current:
            index -= 1
        print((len(scores) - index) + 1)


if __name__ == "__main__":
    main()
