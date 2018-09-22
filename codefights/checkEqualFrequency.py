def checkEqualFrequency(inputArray):
    if len(inputArray)==1:
        return True
    counter={}
    for i in inputArray:
        counter.setdefault(i,0)
        counter[i]+=1
    vals=counter.values()
    return all([item==vals[0] for item in vals])