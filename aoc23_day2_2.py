#!/usr/bin/env python
import math


def parse_line(line):
    header, rest = line.split(':')
    game = int(header.split()[-1])
    sets = []
    for item in rest.split(';'):
        r, g, b = 0, 0, 0
        for color in item.split(','):
            num, name = color.split()
            num = int(num)
            if name == 'red':
                r = num
            elif name == 'green':
                g = num
            elif name == 'blue':
                b = num

        sets.append((r, g, b))

    return game,sets


def get_min(sets):
    res = (max([item[i] for item in sets]) for i in range(3))
    return tuple(res)


def main():
    res = 0
    while True:
        try:
            line = input()
            game, sets = parse_line(line)
            acc = math.prod(get_min(sets))
            res += acc
        except EOFError:
            break

    print(res)


if __name__ == '__main__':
    main()
