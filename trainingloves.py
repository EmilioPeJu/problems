#!/usr/bin/env python


n, b = map(int, input().strip().split())
res = 0
partial_fact = 1

for i in range(2, n+1):
    partial_fact *= i
    while partial_fact % b == 0:
        res += 1
        partial_fact //= b

print(res)
