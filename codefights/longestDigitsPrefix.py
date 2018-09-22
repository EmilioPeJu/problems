def longestDigitsPrefix(inputString):
    if len(inputString)==0:
        return ""
    i=0
    while i<len(inputString) and  inputString[i].isdigit():
        i+=1
    return inputString[:i]
