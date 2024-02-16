#!/usr/bin/env python

def min_val(x, n, best_so_far):
    if n > best_so_far[0]:
        return 16

    if x == 0:
        best_so_far[0] = min(best_so_far[0], n)
        return n

    x &= 32767;
    n1 = min_val(x * 2, n + 1, best_so_far)
    sum_needed = x & -x;
    n2 = min_val(x + sum_needed, n + sum_needed, best_so_far)
    return min(n1, n2)


def main():
    n = int(input())
    for x in [int(item) for item in input().split()]:
        best_so_far = [16]
        print(min_val(x, 0, best_so_far), end=' ')

    print('')


if __name__ == '__main__':
    main()
