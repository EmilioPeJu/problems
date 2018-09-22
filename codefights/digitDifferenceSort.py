def digitDifferenceSort(a):
    def key(i):
        minnum=float("inf")
        maxnum=0
        while i:
            digit=i%10
            minnum=min(minnum,digit)
            maxnum=max(maxnum,digit)
            i/=10
        return maxnum-minnum

    
    b=[(key(item), len(a)-i-1,item) for i,item in enumerate(a)]
    b.sort()
    return [item[2] for item in b]
    