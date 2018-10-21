#!/usr/bin/env python
import json

def get_stdin_from_file():
    import os
    import sys
    TEST_FILE = 'in_test'
    if os.environ.get('DEBUG', '0') != '0' \
            and os.access(TEST_FILE, os.R_OK):
        sys.stdin = open(TEST_FILE, 'r')


def main():
    get_stdin_from_file()
    N = int(input())
    author_citing = {}
    h_index = {}
    for _ in range(N):
        entry = json.loads(input())
        current_citing = entry['citing_paper_count']
        for author in entry['authors']['authors']:
            fullname = author['full_name']
            author_citing.setdefault(fullname, []).append(current_citing)
    for key in author_citing:
        citing = author_citing[key]
        citing.sort(reverse=True)
        i = 0
        while i < len(citing) and (i + 1) <= citing[i]:
            i += 1
        if i == len(citing):
            h_index[key] = i
        else:
            h_index[key] = i
    authors = list(author_citing.keys())

    class ToCompare(object):
        def __init__(self, obj):
            self.obj = obj
        def __lt__(self, other):
            if h_index[self.obj] == h_index[other.obj]:
                return self.obj > other.obj
            return h_index[self.obj] < h_index[other.obj]

    authors.sort(key=ToCompare, reverse=True)
    for author in authors:
        print("{} {}".format(author, h_index[author]))


if __name__ == "__main__":
    main()
