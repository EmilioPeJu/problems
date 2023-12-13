#!/usr/bin/env python


def read_tubes():
    res = []
    while True:
        try:
            res.append(input().strip())
        except EOFError:
            break

    return res


def find_start(tubes):
    for i in range(len(tubes)):
        for j in range(len(tubes[0])):
            if tubes[i][j] == 'S':
                return i, j

    raise Exception('should find start')


NEXT_STEP = {
    (1, 0, '|'): (1, 0),
    (-1, 0, '|'): (-1, 0),
    (0, 1, '-'): (0, 1),
    (0, -1, '-'): (0, -1),
    (0, -1, 'L'): (-1, 0),
    (1, 0, 'L'): (0, 1),
    (0, 1, 'J'): (-1, 0),
    (1, 0, 'J'): (0, -1),
    (0, 1, '7'): (1, 0),
    (-1, 0, '7'): (0, -1),
    (-1, 0, 'F'):  (0, 1),
    (0, -1, 'F'):  (1, 0)
}


def get_next_step(last, c):
    sr, sc = last
    return NEXT_STEP.get((sr, sc, c))


def follow_track(step, current, tubes):
    r, c = current
    path = []
    visited = set()
    while True:
        if r < 0 or r >= len(tubes) or c < 0 or c >= len(tubes[0]):
            return False, []

        if tubes[r][c] == '.':
            return False, []

        if tubes[r][c] == 'S':
            return True, path

        if (r, c) in visited:
            return False, path

        path.append((r, c))
        visited.add((r, c))
        step = get_next_step(step, tubes[r][c])
        if step is None:
            return False, []

        r += step[0]
        c += step[1]


def shoelace(v):
    res = 0
    n = len(v)
    for i in range(n - 1):
        res += v[i][0] * v[i + 1][1] - v[i][1] * v[i + 1][0]

    res += v[n - 1][0] * v[0][1] - v[n - 1][1] * v[0][0]

    if res < 0:
        res *= -1

    return res / 2


def main():
    tubes = read_tubes()
    s_pos = find_start(tubes)
    for step in [(1, 0), (-1,0), (0, 1), (0, -1)]:
        ok, result = follow_track(step,
                                  (s_pos[0] + step[0], s_pos[1] + step[1]),
                                  tubes)
        if ok:
            break

    if not ok:
        raise Exception('No solution found')

    area = shoelace(result)
    # pick's theorem
    result = area - len(result) / 2 + 1
    # ok, the result is not exact, not sure why, but the floored value was
    # the right one
    print(int(result))


if __name__ == '__main__':
    main()
