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


def find_mapped_ranges(current_range, mapx):
    res = []
    start1, len1 = current_range

    while len1 > 0:
        idx = bisect.bisect_right(mapx, start1, key=lambda x: x[0]) - 1
        if idx == -1:
            overlapping = max((start1 + len1 - 1) - mapx[0][0] + 1, 0)
            res.append((start1, len1 - overlapping))
            start1 = mapx[0][0]
            len1 = overlapping
        else:
            start2, start2x, len2 = mapx[idx]
            if start2 + len2 - 1 < start1:
                res.append((start1, len1))
                len1 = 0
            else:
                offset = start1 - start2
                len2 -= offset
                start2x += offset
                overlapping = min(len1, len2)
                res.append((start2x, overlapping))
                start1 += overlapping
                len1 -= overlapping


    return res


def test_find_mapped_ranges():
    assert find_mapped_ranges((40, 9), [(50, 52, 48), (98, 50, 2)]) == [(40, 9)]
    assert find_mapped_ranges((100, 9), [(50, 52, 48), (98, 50, 2)]) == [(100, 9)]
    assert find_mapped_ranges((51, 5), [(50, 52, 48), (98, 50, 2)]) == [(53, 5)]
    assert find_mapped_ranges((96, 4),
                              [(50, 52, 48), (98, 50, 2)]) == [(98, 2), (50, 2)]


def follow_seed_ranges(seed_ranges, maps):
    current_ranges = seed_ranges
    for mapx in maps:
        new_ranges = []
        for current_range in current_ranges:
            ranges = find_mapped_ranges(current_range, mapx)
            new_ranges.extend(ranges)

        current_ranges = new_ranges

    return current_ranges


def main():
    line = input()
    seed_input = [int(item) for item in line.split(':')[-1].split()]
    seed_ranges = [(seed_input[i], seed_input[i + 1])
                    for i in range(0, len(seed_input), 2)]
    # empty line
    input()
    maps = []
    for _ in range(7):
        mapx = read_map()
        maps.append(mapx)

    final_ranges = follow_seed_ranges(seed_ranges, maps)
    res = min(final_ranges)
    print(res[0])


if __name__ == '__main__':
    main()
