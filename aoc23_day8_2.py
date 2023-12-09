#!/usr/bin/env python


def read_graph():
    graph = {}
    while True:
        try:
            line = input()
        except EOFError:
            break

        node, rest = line.split(' = ')
        a, b = rest.split(', ')
        graph[node] = [a.lstrip('('), b.rstrip(')')]

    return graph


def find_length(current, instructions, graph):
    # I took a small leap of faith thinking that the path from the start to the
    # first time we find the target forms a cycle, it turned out to work
    i = 0
    n = len(instructions)
    while not current.endswith('Z'):
        inst = instructions[i % n]
        index = 0 if inst == 'L' else 1
        current = graph[current][index]
        i += 1

    return i


def gcd(a, b):
    if a > b:
        a, b = b, a

    while a > 0:
        a, b = b % a, a

    return b


def lcm(a, b):
    return a * b // gcd(a, b)


def solve(instructions, graph):
    currents = [item for item in graph if item.endswith('A')]
    paths_length = []
    for current in currents:
        paths_length.append(find_length(current, instructions, graph))

    res = paths_length[0]
    for x in paths_length[1:]:
        res = lcm(res, x)

    return res


def main():
    instructions = input()
    # empty line
    input()
    graph = read_graph()
    res = solve(instructions, graph)
    print(res)


if __name__ == '__main__':
    main()
