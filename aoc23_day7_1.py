#!/usr/bin/env python

class Hand(object):
    OPTIONS = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']

    def __init__(self, hand):
        self.hand = hand

    @staticmethod
    def score(hand):
        counts = {}
        for c in hand:
            counts.setdefault(c, 0)
            counts[c] += 1

        count_counts = {}
        for count in counts.values():
            count_counts.setdefault(count, 0)
            count_counts[count] += 1

        if count_counts.get(5, 0) == 1:
            return 7
        elif count_counts.get(4, 0) == 1:
            return 6
        elif count_counts.get(3, 0) == 1:
            if count_counts.get(2, 0) == 1:
                return 5
            else:
                return 4
        elif count_counts.get(2, 0) == 2:
            return 3
        elif count_counts.get(2, 0) == 1:
            return 2
        else:
            return 1

    def __lt__(self, other):
        key1 = [self.score(self.hand)] + \
               [self.OPTIONS.index(c) for c in self.hand]
        key2 = [self.score(other.hand)] + \
               [self.OPTIONS.index(c) for c in other.hand]
        return key1 < key2


def read_input():
    res = []
    while True:
        try:
            line = input()
            hand, bid = line.split()
            res.append((Hand(hand), int(bid)))
        except EOFError:
            break

    return res


def main():
    hands_bids = read_input()
    hands_bids.sort()
    res = 0
    for i, hand_bid in enumerate(hands_bids):
        res += (i + 1) * hand_bid[1]

    print(res)


if __name__ == '__main__':
    main()
