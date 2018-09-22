def houseNumbersSum(inputArray):
    ans=0
    for i in inputArray:
        if i==0:
            break
        else:
            ans+=i
    return ans

