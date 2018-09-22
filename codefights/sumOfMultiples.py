def sumOfMultiples(n, k):
    i=1
    res=0
    while i*k<=n:
        res+=i*k
        i+=1
    return res
