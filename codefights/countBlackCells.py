def countBlackCells(n, m):
    ans=0
    for line in range(n):
        upper=(line * m) / n 
        if (line*m) % n==0 and upper>0:
            upper-=1
        lower=((line+1)*m) / n + 1
        ans+=(lower-upper) 
    return ans-1