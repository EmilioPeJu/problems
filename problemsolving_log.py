#!/usr/bin/env python

def solve1():
    count = [0] * 26
    n = int(input())
    for c in input().strip():
        count[ord(c) - ord('A')] += 1

    res = 0
    for i in range(len(count)):
        if count[i] >= i + 1:
            res += 1

    print(res)


def main():
    t = int(input())
    for _ in range(t):
        solve1()


if __name__ == '__main__':
    main()
