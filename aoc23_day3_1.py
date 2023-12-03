#!/usr/bin/env python


def read_schematic():
    res = []
    while True:
        try:
            res.append(input().strip())
        except EOFError:
            break

    return res


def expand_adj(schem, i, j):
    res = []
    for step_i, step_j in ((-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1),
                           (1, -1), (1, 1)):
        new_i = i + step_i
        new_j = j + step_j
        if new_i >= 0 and new_i < len(schem) and new_j >= 0 \
                and new_j < len(schem[0]):
            res.append((new_i, new_j))

    return res


def is_valid(schem, coords):
    for (i1, j1) in coords:
        for (i2, j2) in expand_adj(schem, i1, j1):
            if not schem[i2][j2].isdigit() and schem[i2][j2] != '.':
                return True

    return False


def find_valid_number(schem, i, j, visited):
    coords = []
    num = 0
    line = schem[i]
    k = j
    while k < len(line) and line[k].isdigit():
        num = num * 10 + int(line[k])
        coords.append((i, k))
        visited[i][k] = True
        k += 1

    if is_valid(schem, coords):
        return num
    else:
        return 0


def main():
    schem = read_schematic()
    visited = [[False] * len(schem[0]) for _ in range(len(schem))]
    res = 0
    for i, line in enumerate(schem):
        for j, char in enumerate(line):
            if visited[i][j]:
                continue

            res += find_valid_number(schem, i, j, visited)

    print(res)


if __name__ == '__main__':
    main()
