#!/usr/bin/env python


def solve1(time, dist):
    total = 0
    for i in range(time):
        v = i
        d = (time - i) * v
        if d > dist:
            total += 1

    return total


def main():
    times = [int(item) for item in input().split(':')[-1].split()]
    dists = [int(item) for item in input().split(':')[-1].split()]
    res = 1
    for time, dist in zip(times, dists):
        res *= solve1(time, dist)

    print(res)

if __name__ == '__main__':
    main()
