def maxDivisor(left, right, divisor):
    for i in range(right,left-1,-1):
        if i%divisor==0:
            return i
    return -1
