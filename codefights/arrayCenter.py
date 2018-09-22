def arrayCenter(A):
    minA=min(A)
    avgA=float(sum(A))/len(A)
    res=[]
    for i in A:
        if abs(i-avgA)<minA:
            res.append(i)
    return res