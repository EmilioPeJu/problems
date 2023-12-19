#!/usr/bin/env python
GRID_SIZE = 800


def is_valid(grid, i, j):
    return i >= 0 and i < len(grid) and j >=0 and j < len(grid[0])


def flood(grid, pos, val, new_val):
    frontier = [pos]
    while frontier:
        r, c = frontier.pop()
        grid[r][c] = new_val
        for step in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            new_r = r + step[0]
            new_c = c + step[1]
            if is_valid(grid, new_r, new_c) and grid[new_r][new_c] == val:
                frontier.append((new_r, new_c))


def count(grid):
    res = 0
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if grid[i][j] == 1 or grid[i][j] == 0:
                res += 1

    return res


def main():
    grid = [[0] * GRID_SIZE for _ in range(GRID_SIZE)]
    pos = [GRID_SIZE // 2, GRID_SIZE // 2]
    while True:
        try:
            c, num, _ = input().strip().split()
        except EOFError:
            break

        num = int(num)
        step = {'U': (-1, 0), 'D': (1, 0), 'L': (0, -1), 'R': (0, 1)}.get(c)
        for _ in range(num):
            grid[pos[0]][pos[1]] = 1
            pos[0] += step[0]
            pos[1] += step[1]

    flood(grid, (0, 0), 0, 2)
    res = count(grid)
    print(res)


if __name__ == '__main__':
    main()
