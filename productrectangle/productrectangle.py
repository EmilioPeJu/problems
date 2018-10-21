#!/usr/bin/env python


def get_stdin_from_file():
    import os
    import sys
    TEST_FILE = 'in1'
    if os.environ.get('DEBUG', '0') != '0' \
            and os.access(TEST_FILE, os.R_OK):
        sys.stdin = open(TEST_FILE, 'r')

def find_max_sum(C):
    aux = [[0]*len(C[0]) for _ in range(len(C))]
    for i in range(len(aux)):
        for j in range(len(aux[0])):
            aux[i][j] += C[i][j]
            if i > 0:
                aux[i][j] += aux[i-1][j]
            if j > 0:
                aux[i][j] += aux[i][j-1]
            if i > 0 and j > 0:
                aux[i][j] -= aux[i-1][j-1]
    max_sum = -1000000
    for istart in range(len(aux)):
        for iend in range(istart, len(aux)):
            for jstart in range(len(aux[0])):
                for jend in range(jstart, len(aux[0])):
                    current_sum = aux[iend][jend]
                    if jstart > 0:
                        current_sum -= aux[iend][jstart-1]
                    if istart > 0:
                        current_sum -= aux[istart - 1][jend]
                    if jstart > 0 and istart > 0:
                        current_sum += aux[istart-1][jstart-1]
                    max_sum = max(max_sum, current_sum)
    return max_sum


def main():
    get_stdin_from_file()
    n, m = [int(i) for i in input().strip().split()]
    A = [int(i) for i in input().strip().split()]
    B = [int(i) for i in input().strip().split()]
    C = [[x*y for y in B] for x in A] 
    res = find_max_sum(C)
    print(res)

if __name__ == "__main__":
    main()
