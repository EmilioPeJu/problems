#!/usr/bin/env python
from collections import OrderedDict


def the_hash(item):
    res = 0
    for c in item:
        res = ((res + ord(c)) * 17) & 255

    return res


def main():
    line = input()
    boxes = [OrderedDict() for _ in range(256)]
    for command in line.split(','):
        if '=' in command:
            lens, focal = command.split('=')
            box_i = the_hash(lens)
            boxes[box_i][lens] = int(focal)
        elif command.endswith('-'):
            lens = command[:-1]
            box_i = the_hash(lens)
            if lens in boxes[box_i]:
                del boxes[box_i][lens]

    res = 0
    for i in range(256):
        box = boxes[i]
        for j, focal in enumerate(box.values()):
            res += (1 + i) * (1 + j) * focal

    print(res)


if __name__ == '__main__':
    main()
