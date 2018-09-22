def isSumOfConsecutive2(n):
    left=1
    right=2
    ans=0
    for left in range(1,n):
        right=left
        res=0
        while True:
            right+=1
            res=(right-left+1)*(left+right)/2
            if res>=n:
                break
        if res==n:
            ans+=1
    return ans