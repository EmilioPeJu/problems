#!/usr/bin/env python


def parse_line(line):
    without_header = line.split(':')[-1]
    parts = without_header.split('|')
    wins = [int(i) for i in parts[0].split()]
    got = [int(i) for i in parts[1].split()]
    return wins, got


def main():
    all_matches = []
    while True:
        try:
            line = input()
            wins, got = parse_line(line)
            wins = set(wins)
            matches = 0
            for item in got:
                if item in wins:
                    matches += 1

            all_matches.append(matches)
        except EOFError:
            break

    counts = [1] * len(all_matches)
    for i, v in enumerate(all_matches):
        for j in range(i + 1, i + 1 + v):
            counts[j] += counts[i]

    print(sum(counts))


if __name__ == '__main__':
    main()
