#!/usr/bin/env python
import sys


def read_maze():
    res = []
    while True:
        try:
            res.append(input().strip())
        except EOFError:
            break

    return res


def _dfs(pos, target, maze, visited, depth, longest):
    if pos in visited:
        return

    visited.add(pos)

    if pos == target:
        if depth > longest[0]:
            longest[0] = depth

    for step in ((1, 0), (-1, 0), (0, 1), (0, -1)):
        new_r = pos[0] + step[0]
        new_c = pos[1] + step[1]
        if new_r >= 0 and new_r < len(maze) \
                and new_c >= 0 and new_c < len(maze[0]) \
                and maze[new_r][new_c] != '#':
            new_depth = depth + 1
            if maze[new_r][new_c] == 'v':
                new_r += 1
                new_depth += 1
            elif maze[new_r][new_c] == '>':
                new_c += 1
                new_depth += 1
            elif maze[new_r][new_c] == '<':
                new_c -= 1
                new_depth += 1
            elif maze[new_r][new_c] == '^':
                new_r -= 1
                new_depth += 1
            _dfs((new_r, new_c), target, maze, visited, new_depth, longest)

    visited.remove(pos)


def dfs(start, target, maze):
    visited = set()
    longest = [0]
    _dfs(start, target, maze, visited, 0, longest)
    return longest[0]


def main():
    maze = read_maze()
    start = (0, 1)
    target = (len(maze) - 1, len(maze[0]) - 2)
    sys.setrecursionlimit(1000000)
    res = dfs(start, target, maze)
    print(res)

if __name__ == '__main__':
    main()
