#!/usr/bin/env python
import random


class UF(object):
    def __init__(self):
        self.parent = {}
        self.size = {}

    def find(self, node):
        if node not in self.parent:
            self.parent[node] = node
            self.size[node] = 1

        while self.parent[node] != node:
            node = self.parent[node]

        return node

    def union(self, a, b):
        root_a = self.find(a)
        root_b = self.find(b)

        if self.size[root_a] >= self.size[root_b]:
            self.parent[root_b] = root_a
            self.size[root_a] += self.size[root_b]
        else:
            self.parent[root_a] = root_b
            self.size[root_b] += self.size[root_a]


def read_graph():
    edges = []
    nodes = set()
    while True:
        try:
            parts = input().strip().split()
            node = parts[0][:-1]
            nodes.add(node)
            for dst in parts[1:]:
                edges.append((node, dst))
                nodes.add(dst)
        except EOFError:
            break

    return edges, nodes


def main():
    edges, nodes = read_graph()
    while not kargers(edges, nodes):
        pass


def kargers(edges, nodes):
    # randomize edges iteration
    edges.sort(key=lambda _: random.random())
    n_vertices = len(nodes)
    uf = UF()
    for u, v in edges:
        if uf.find(u) != uf.find(v):
            uf.union(u, v)
            n_vertices -= 1
            if n_vertices == 2:
                break

    min_cut = 0
    for u, v in edges:
        if uf.find(u) != uf.find(v):
            min_cut += 1

    if min_cut != 3:
        # need to retry from start
        return False

    components = {}
    for node in nodes:
        root_node = uf.find(node)
        count = components.get(root_node, 0)
        components[root_node] = count + 1

    res = 1
    for _k, v in components.items():
        res *= v

    print(res)
    return True

if __name__ == '__main__':
    main()
