def reduceString(inputString):
    i=0
    while 2*i<len(inputString) and inputString[i]==inputString[len(inputString)-i-1]:
        i+=1
    if 2*i >=len(inputString):
        return ""
    res=inputString[i:len(inputString)-i]
    return res
