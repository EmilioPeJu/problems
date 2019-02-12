#!/usr/bin/env python

t = int(input())
while t:
    t -= 1
    a, b = map(int, input().strip().split())
    if a < b:
        print('<')
    elif a > b:
        print('>')
    else:
        print('=')
