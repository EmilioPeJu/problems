def isIncreasingDigitsSequence(n):
    def getDigits(num):
        res=[]
        while num:
            res.append(num%10)
            num/=10
        res.reverse()
        return res
    digits=getDigits(n)
    for i in range(1,len(digits)):
        if digits[i]<=digits[i-1]:
            return False
    return True
