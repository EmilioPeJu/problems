def arrayMaximalAdjacentDifference(inputArray):
    ans=0
    for i in range(len(inputArray)-1):
        ans=max(ans,abs(inputArray[i]-inputArray[i+1]))
    return ans

