#!/usr/bin/env python

def read_diagram():
    res = []
    while True:
        try:
            line = input().strip()
            if line == '':
                return res

            res.append(line)
        except EOFError:
            return res


def traspose(diagram):
    res = []
    for i in range(len(diagram[0])):
        res.append(''.join([diagram[j][i] for j in range(len(diagram))]))

    return res


def get_range(diagram, i):
    res = 0
    while True:
        i1 = i - res
        i2 = i + res + 1
        if i1 >= 0 and i2 < len(diagram) and diagram[i1] == diagram[i2]:
            res += 1
        else:
            break

    return res


def find_mirror_point(diagram):
    for i in range(len(diagram) - 1):
        count = get_range(diagram, i)
        if count != 0 and count == min((i + 1, len(diagram) - i - 1)):
            return i


def main():
    res = 0
    while True:
        diagram = read_diagram()
        if not diagram:
            break

        x = find_mirror_point(diagram)
        if x is not None:
            res += 100 * (x + 1)
        else:
            tr = traspose(diagram)
            y = find_mirror_point(tr)
            assert y is not None
            res += (y + 1)

    print(res)


if __name__ == '__main__':
    main()
