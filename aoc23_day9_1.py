#!/usr/bin/env python


def predict_next(values):
    all_seqs = []
    while not all((item == 0 for item in values)):
        all_seqs.append(values)
        next_values = []
        for i in range(1, len(values)):
            next_values.append(values[i] - values[i - 1])

        values = next_values

    res = 0
    for i in reversed(range(len(all_seqs))):
        res += all_seqs[i][-1]

    return res


def main():
    res = 0
    while True:
        try:
            values = [int(item) for item in input().split()]
            res += predict_next(values)
        except EOFError:
            break

    print(res)


if __name__ == '__main__':
    main()
