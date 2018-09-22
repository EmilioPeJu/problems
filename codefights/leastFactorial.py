def leastFactorial(n):
    a=1
    fac=1
    while fac<n:
        a+=1
        fac*=a
    return fac