def allLongestStrings(inputArray):
    ans=[]
    lastlongest=0
    for i in inputArray:
        if len(i)>lastlongest:
            ans=[i]
            lastlongest=len(i)
        elif len(i)==lastlongest:
            ans.append(i)
    return ans

