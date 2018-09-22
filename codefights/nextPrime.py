def isPrime(n):
    for i in range(2,int(math.sqrt(n))+1):
        if n%i==0:
            return False
    return True
def nextPrime(n):
    n+=1
    while not isPrime(n):
        n+=1
    return n
