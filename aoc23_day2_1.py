#!/usr/bin/env python


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


def less_or_equal(a, b):
    return all((t[0] <= t[1] for t in zip(a, b)))


def main():
    target = (12, 13, 14)
    res = 0
    while True:
        try:
            line = input()
            game, sets = parse_line(line)
            if all([less_or_equal(i, target) for i in sets]):
                res += game

        except EOFError:
            break

    print(res)


if __name__ == '__main__':
    main()
