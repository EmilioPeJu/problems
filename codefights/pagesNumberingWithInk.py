def pagesNumberingWithInk(current, numberOfDigits):
    def getLen(page):
        return len(str(page))
    def nextBig(page):
        return int("1"+"0"*getLen(page))
    currentLen = getLen(current)
    while True:
        maxAdvanceCurrentLen=nextBig(current)-current
        advance=min(maxAdvanceCurrentLen, numberOfDigits/currentLen)
        print(advance)
        numberOfDigits-=advance*currentLen
        current+=advance
        if numberOfDigits<currentLen:
            return current-1
        currentLen=getLen(current)
        