def checkFactorial(n):
    aux=1
    i=1
    while aux<n:
        i+=1
        aux*=i
    return aux==n
