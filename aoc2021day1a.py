#!/usr/bin/env python

def main():
    last = None
    result = 0
    while True:
        try:
            current = int(input())
            if last and current > last:
                result += 1
            last = current
        except EOFError:
            break
    print(result)


if __name__ == "__main__":
    main()
