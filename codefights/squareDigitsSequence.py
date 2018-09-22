def squareDigitsSequence(a0):
    def sqr_digits(n):
        res = 0
        while n:
            res += (n % 10)**2
            n /= 10
        return res
    res = 1
    numbers = set()
    current = a0
    while current not in numbers:
        res += 1
        numbers.add(current)
        current = sqr_digits(current)
    return res