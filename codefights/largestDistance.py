def largestDistance(A):
    res=0
    for i in range(0,len(A),2):
        for j in range(i+2,len(A),2):
            res=max(res, abs(A[i]-A[j]))
            res=max(res, abs(A[i+1]-A[j+1]))
    return res