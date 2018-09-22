def powersOfTwo(n):
    res = []
    i = 0
    while n:
        if n%2 == 1:
            res.append(2**i)
        n /= 2
        i+=1
    return res