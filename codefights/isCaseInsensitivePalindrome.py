def isCaseInsensitivePalindrome(inputString):
    changes=0
    for i in range(len(inputString)/2+len(inputString)%2):
        if inputString[i].lower()!=inputString[len(inputString)-i-1].lower():
            return False
    return True
