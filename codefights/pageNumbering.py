def pagesNumbering(n)
    p = 1
    d = 1
    result = 0
    while p * 10 <= n
        result += p * 9 * d
        d += 1
        p *= 10
    result = result + (n - p + 1) * d
    return result
