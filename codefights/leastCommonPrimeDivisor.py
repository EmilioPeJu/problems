def isPrime(n):
    import math
    for i in range(2,int(math.sqrt(n))+1):
        if not n%i:
            return False
    return True

def leastCommonPrimeDivisor(a, b):
    candidate=min(a,b)
    for i in range(2,candidate):
        if not a%i and not b%i and isPrime(i):
            return i
    return -1
