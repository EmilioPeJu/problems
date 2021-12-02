#!/usr/bin/env python

def main():
    result = 0
    a = int(input())
    b = int(input())
    c = int(input())
    while True:
        try:
            new_c = int(input())
            if new_c > a:
                result += 1
            a, b, c = b, c, new_c
        except EOFError:
            break
    print(result)


if __name__ == "__main__":
    main()
