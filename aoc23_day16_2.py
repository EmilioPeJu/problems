#!/usr/bin/env python
from collections import deque

def read_diagram():
    res = []
    while True:
        try:
            res.append(input().strip())
        except EOFError:
            break

    return res


def is_valid(diagram, i, j):
    return i >= 0 and i < len(diagram) and j >= 0 and j < len(diagram[0])


def solve1(diagram, start_pos, start_step):
    start = (start_pos, start_step)
    frontier = deque([start])
    explored = set()
    energised = [[0] * len(diagram[0]) for _ in range(len(diagram))]
    while frontier:
        current = frontier.pop()
        pos, step = current
        energised[pos[0]][pos[1]] = 1
        action = diagram[pos[0]][pos[1]]
        no_change = True
        if action == '\\' \
                or action == '|' and step[0] == 0 \
                or action == '-' and step[1] == 0:
            no_change = False
            next_step = (step[1], step[0])
            next_pos = (pos[0] + next_step[0], pos[1] + next_step[1])
            if is_valid(diagram, next_pos[0], next_pos[1]) \
                    and (next_pos, next_step) not in explored:
                frontier.appendleft((next_pos, next_step))
                explored.add((next_pos, next_step))

        if action == '/' \
                or action == '|' and step[0] == 0 \
                or action == '-' and step[1] == 0:
            no_change = False
            next_step = (-step[1], -step[0])
            next_pos = (pos[0] + next_step[0], pos[1] + next_step[1])
            if is_valid(diagram, next_pos[0], next_pos[1]) \
                    and (next_pos, next_step) not in explored:
                frontier.appendleft((next_pos, next_step))
                explored.add((next_pos, next_step))

        if no_change:
            next_step = (step[0], step[1])
            next_pos = (pos[0] + next_step[0], pos[1] + next_step[1])
            if is_valid(diagram, next_pos[0], next_pos[1]) \
                    and (next_pos, next_step) not in explored:
                frontier.appendleft((next_pos, next_step))
                explored.add((next_pos, next_step))

    count = 0
    for item in energised:
        count += sum(item)

    return count


def main():
    diagram = read_diagram()
    res = 0
    for i in range(len(diagram)):
        count = solve1(diagram, (i, 0), (0, 1))
        res = max(res, count)
        count = solve1(diagram, (i, len(diagram[0]) - 1), (0, -1))
        res = max(res, count)

    for i in range(len(diagram[0])):
        count = solve1(diagram, (0, i), (1, 0))
        res = max(res, count)
        count = solve1(diagram, (len(diagram) - 1, i), (-1, 0))
        res = max(res, count)

    print(res)


if __name__ == '__main__':
    main()
