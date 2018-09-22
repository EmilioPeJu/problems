def longestSequence(A):
    ans=0
    if len(A)==1:
        return 1
    for i in range(len(A)-1):
        for j in range(i+1,len(A)):
            r=A[j]-A[i]
            index=2
            current=2
            for k in range(j+1,len(A)):
                if A[k]==(index*r+A[i]):
                    current+=1
                    index+=1
            ans=max(ans,current)
    return ans