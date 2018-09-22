def countSumOfTwoRepresentations2(n, l, r):
    res=0
    for i in range(l,r+1):
        if (n-i)>=i and (n-i)<=r:
            res+=1
    return res
