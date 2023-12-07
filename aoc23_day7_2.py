#!/usr/bin/env python

class Hand(object):
    OPTIONS = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A']

    def __init__(self, hand):
        self.hand = hand

    def __str__(self):
        return self.hand

    @staticmethod
    def score(hand):
        counts = {}
        n_js = 0
        max_c = None
        max_count = 0
        for c in hand:
            if c == 'J':
                n_js += 1
            else:
                counts.setdefault(c, 0)
                counts[c] += 1
                if counts[c] > max_count:
                    max_c = c
                    max_count = counts[c]

        if n_js == 5:
            return [1, 0, 0, 0, 0] + [Hand.OPTIONS.index(c) for c in hand]


        counts[max_c] += n_js
        count_counts = [0] * 6
        for count in counts.values():
            count_counts[count] += 1

        return list(reversed(count_counts))+ [Hand.OPTIONS.index(c) for c in hand]

    def __lt__(self, other):
        return self.score(self.hand) < self.score(other.hand)


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
