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


def follow_until_zzz(instructions, graph):
    i = 0
    current = 'AAA'
    while current != 'ZZZ':
        inst = instructions[i % len(instructions)]
        index = 0 if inst == 'L' else 1
        current = graph[current][index]
        i += 1

    return i


def main():
    instructions = input()
    # empty line
    input()
    graph = read_graph()
    res = follow_until_zzz(instructions, graph)
    print(res)


if __name__ == '__main__':
    main()
