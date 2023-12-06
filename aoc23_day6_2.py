#!/usr/bin/env python
import math


def solve1(time, dist):
    total = 0
    # i * (time - i) = dist
    # time - i**2 - dist = 0
    # i**2 - time*i + dist
    x = math.sqrt(time*time - 4*dist) / 2
    i1 = int(time / 2 - x)
    while i1 * (time - i1) < dist:
        i1 += 1

    i2 = int(time / 2 + x)
    while i2 * (time - i2) > dist:
        i2 += 1

    return i2 - i1


def main():
    time = int(input().split(':')[-1].replace(" ", ""))
    dist = int(input().split(':')[-1].replace(" ", ""))
    res = solve1(time, dist)
    print(res)


if __name__ == '__main__':
    main()
