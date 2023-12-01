#!/usr/bin/env python
import re

NUMBERS = [
    'zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven',
    'eight', 'nine'
]
MASTER_REGEX = '|'.join(NUMBERS) + '|' + '|'.join([str(i) for i in range(10)])


def main():
    res = 0
    while True:
        try:
            line = input()
            m1 = re.search(MASTER_REGEX, line)
            d1 = m1.group()
            if d1.isdigit():
                res += 10 * int(d1)
            else:
                res += 10 * int(NUMBERS.index(d1))

            m2 = re.search(MASTER_REGEX[::-1], line[::-1])
            d2 = m2.group()[::-1]
            if d2.isdigit():
                res += int(d2)
            else:
                res += int(NUMBERS.index(d2))
        except EOFError:
            break

    print(res)

if __name__ == '__main__':
    main()
