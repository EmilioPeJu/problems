def fibonacciIndex(N):
    last=0
    current=1
    index=1
    while len(str(current))<N:
        index+=1
        last,current=current,last+current
    return index
