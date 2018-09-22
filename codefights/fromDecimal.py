def fromDecimal(base, n):
    res = []
    while n:
        res.append(n % base)
        n /= base
    return ''.join(map(str, reversed(res)))