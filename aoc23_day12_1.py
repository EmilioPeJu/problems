#!/usr/bin/env python

def make_pattern(line):
    i = 0
    cmp = []
    while i < len(line):
        if line[i] == '.':
            i += 1
            continue

        cnt = 0
        while i < len(line) and line[i] == '#':
            cnt += 1
            i += 1

        cmp.append(cnt)

    return cmp


def test_make_pattern():
    assert make_pattern('#.#.###') == [1, 1, 3]
    assert make_pattern('#....######..#####.') == [1, 6, 5]
    assert make_pattern('.###.##....#') ==  [3, 2, 1]


def brute_force(current, left, target):
    if left == '':
        return int(make_pattern(current) == target)

    if left[0] == '?':
        res = 0
        res += brute_force(current + '.', left[1:], target)
        res += brute_force(current + '#', left[1:], target)
        return res
    else:
        return brute_force(current + left[0], left[1:], target)


def main():
    res = 0
    while True:
        try:
            line, nums = input().split()
            nums = [int(item) for item in nums.split(',')]
            count = brute_force('', line, nums)
            res += count
        except EOFError:
            break

    print(res)


if __name__ == '__main__':
    main()
