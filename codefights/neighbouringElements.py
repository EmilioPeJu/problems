def neighbouringElements(a):
    ans=0
    for i in range(len(a)):
        for j in range(len(a[0])):
            if (i+1)<len(a) and a[i][j]==a[i+1][j]:
                ans+=1
            if (j+1)<len(a[0]) and a[i][j]==a[i][j+1]:
                ans+=1
    return ans
