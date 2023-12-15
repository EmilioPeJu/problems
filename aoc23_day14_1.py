#!/usr/bin/env python

def read_diagram():
    res = []
    while True:
        try:
            res.append(input().strip())
        except EOFError:
            break

    return res


def calculate_column(column):
    res = 0
    last = 0
    for i, c in enumerate(column):
        if c == '#':
            last = i + 1
        elif c == 'O':
            res += len(column) - last
            last += 1

    return res


def main():
   diagram = read_diagram() 
   res = 0
   for j in range(len(diagram[0])):
       column = ''.join([diagram[i][j] for i in range(len(diagram))])
       res += calculate_column(column)

   print(res)


if __name__ == '__main__':
    main()
