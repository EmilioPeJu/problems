def countNeighbouringPairs(inputString):
    if inputString=="":
        return ""
    i=1
    ans=0
    while i<len(inputString):
        if inputString[i]==inputString[i-1]:
            ans+=1
        i+=1
    return ans
