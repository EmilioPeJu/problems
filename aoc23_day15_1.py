#!/usr/bin/env python


def the_hash(item):
    res = 0
    for c in item:
        res = ((res + ord(c)) * 17) & 255

    return res


def main():
    line = input()
    res = 0
    for item in line.split(','):
        res += the_hash(item)

    print(res)


if __name__ == '__main__':
    main()
