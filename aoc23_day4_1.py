#!/usr/bin/env python

def parse_line(line):
    without_header = line.split(':')[-1]
    parts = without_header.split('|')
    wins = [int(i) for i in parts[0].split()]
    got = [int(i) for i in parts[1].split()]
    return wins, got

def main():
    res = 0
    while True:
        try:
            line = input()
            wins, got = parse_line(line)
            wins = set(wins)
            points = 1
            for item in got:
                if item in wins:
                    points *= 2

            points //= 2
            res += points
        except EOFError:
            break

    print(res)


if __name__ == '__main__':
    main()
