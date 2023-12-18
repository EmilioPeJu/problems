#!/usr/bin/env python
from heapq import heappop, heappush


def read_heat_map():
    res = []
    while True:
        try:
            res.append([int(item) for item in input().strip()])
        except EOFError:
            break

    return res


def is_valid(heat, pos):
    return pos[0] >= 0 and pos[0] < len(heat) \
        and pos[1] >= 0 and pos[1] < len(heat[0])


def expand(heat, dist, pos, direction, times):
    for new_dir in (direction,
                    (direction[1], direction[0]),
                    (-direction[1], -direction[0])):
        new_pos = (pos[0] + new_dir[0], pos[1] + new_dir[1])
        if new_dir == direction:
            new_times = times + 1
        else:
            new_times = 1

        if new_times > 3 or not is_valid(heat, new_pos):
            continue

        new_dist = dist + heat[new_pos[0]][new_pos[1]]
        yield new_dist, new_pos, new_dir, new_times


def ucs(heat):
    # distance, position, last direction, times for last direction
    frontier = [(0, (0, 0), (0, 1), 0)]
    target = (len(heat) - 1, len(heat[0]) - 1)
    visited = set()
    current_pos = (0, 0)
    while current_pos != target:
        dist, current_pos, direction, times = heappop(frontier)
        if (current_pos, direction, times) in visited:
            continue

        visited.add((current_pos, direction, times))
        for new_dist, new_pos, new_dir, new_times in expand(heat, dist,
                                                            current_pos,
                                                            direction,
                                                            times):
            heappush(frontier, (new_dist, new_pos, new_dir, new_times))

    return dist


def main():
    heat = read_heat_map()
    dist = ucs(heat)
    print(dist)


if __name__ == '__main__':
    main()
