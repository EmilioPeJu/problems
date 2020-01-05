#!/usr/bin/env python
import numpy as np
SIZE = 16384

def draw(grid, wire, wire_id, crossed):
    x, y = grid.shape
    # safe initial position
    x //= 2
    y //= 2
    dirs = {'U': (-1, 0), 'D': (1, 0), 'L': (0, -1), 'R': (0, 1)}
    for part in wire:
        direction = part[0]
        number = int(part[1:])
        for step in range(number):
            x += dirs[direction][0]
            y += dirs[direction][1]
            if grid[x][y] != 0 and grid[x][y] != wire_id:
                crossed.append((x, y))
            grid[x][y] |= wire_id


def key_man(pos):
    return abs(pos[0] - SIZE // 2) + abs(pos[1] - SIZE // 2)


def main():
    grid = np.zeros((SIZE, SIZE), dtype='int8')
    wire_a = input().strip().split(',')
    wire_b = input().strip().split(',')
    crossed = []
    draw(grid, wire_a, 1, [])
    draw(grid, wire_b, 2, crossed)
    result = min(crossed, key=key_man)
    print(result, abs(result[0] - SIZE // 2) + abs(result[1] - SIZE // 2))


if __name__ == "__main__":
    main()
