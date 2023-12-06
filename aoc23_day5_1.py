#!/usr/bin/env python
import bisect


def read_map():
    res = []
    # header
    input()
    while True:
        try:
            line = input()
            if line.strip() == '':
                break

            a, b, c = [int(item) for item in line.split()]
            res.append((b, a, c))
        except EOFError:
            break

    res.sort()
    return res


def convert(item, map_entry):
    init_start, end_start, length = map_entry
    offset = item - init_start
    if offset >= 0 and offset < length:
        return end_start + offset
    else:
        return item


def follow_seed(seed, maps):
    item = seed
    for mapx in maps:
        idx = bisect.bisect_right(mapx, (item, 0, 0)) - 1
        item = convert(item, mapx[idx])

    return item


def main():
    line = input()
    seeds = [int(item) for item in line.split(':')[-1].split()]
    # empty line
    input()
    maps = []
    for _ in range(7):
        mapx = read_map()
        maps.append(mapx)

    res = min((follow_seed(seed, maps) for seed in seeds))
    print(res)


if __name__ == '__main__':
    main()
