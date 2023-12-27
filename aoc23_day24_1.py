#!/usr/bin/env python
import numpy as np


def read_equations():
    equs = []
    const = []
    while True:
        try:
            pos, vel = input().strip().split(' @ ')
            pos = tuple(int(i) for i in pos.split(', '))
            vel = tuple(int(i) for i in vel.split(', '))
            # pos[0] + vel[0] * t = x
            # pos[1] + vel[1] * t = y
            # pos[0] + vel[0] (y - pos[1]) / vel[1] = x
            # pos[0] vel[1] + vel[0] y - vel[0] pos[1] - vel[1] x = 0
            #  - vel[1] x + vel[0] y + pos[0] vel[1] - pos[1] vel[0] = 0
            equs.append([-vel[1], vel[0], pos[0]* vel[1] - pos[1] * vel[0]])
            # to check that t >= 0
            const.append((pos[0], vel[0]))
        except EOFError:
            break

    return equs, const


def is_valid(val):
    return val >= 200000000000000 and val <= 400000000000000


def main():
    res = 0
    equations, const = read_equations()
    for i in range(len(equations)):
        for j in range(i + 1, len(equations)):
            a = np.array([equations[i][0:2], equations[j][0:2]])
            b = np.array([-equations[i][2], - equations[j][2]])
            try:
                sol = np.linalg.solve(a, b)
            except np.linalg.LinAlgError:
                continue

            if is_valid(sol[0]) and is_valid(sol[1]) \
                    and (sol[0] - const[i][0]) / const[i][1] >= 0 \
                    and (sol[0] - const[j][0]) / const[j][1] >= 0:
                res += 1

    print(res)

if __name__ == '__main__':
    main()
