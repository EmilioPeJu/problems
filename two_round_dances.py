#!/usr/bin/env python
import math


def main():
    n = int(input())
    res = int(math.factorial(n - 1)) // (n // 2)
    print(res)
    

if __name__ == '__main__':
    main()
