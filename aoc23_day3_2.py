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


def find_adj_gears(schem, coords):
    res = []
    for (i1, j1) in coords:
        for (i2, j2) in expand_adj(schem, i1, j1):
            if schem[i2][j2] == '*':
                res.append((i2, j2))

    return list(set(res))


def find_valid_number(schem, i, j, visited):
    coords = []
    num = 0
    line = schem[i]
    if not schem[i][j].isdigit():
        return None, []

    k = j
    while k < len(line) and line[k].isdigit():
        num = num * 10 + int(line[k])
        coords.append((i, k))
        visited[i][k] = True
        k += 1

    return num, find_adj_gears(schem, coords)


def main():
    schem = read_schematic()
    visited = [[False] * len(schem[0]) for _ in range(len(schem))]
    gears_ratio = {}
    for i, line in enumerate(schem):
        for j, char in enumerate(line):
            if visited[i][j]:
                continue

            num, gears = find_valid_number(schem, i, j, visited)
            if num is None:
                continue

            for gear in gears:
                if gear not in gears_ratio:
                    gears_ratio[gear] = [0, 1]

                gears_ratio[gear][0] += 1
                gears_ratio[gear][1] *= num

    res = 0
    for count, ratio  in gears_ratio.values():
        if count == 2:
            res += ratio

    print(res)


if __name__ == '__main__':
    main()
