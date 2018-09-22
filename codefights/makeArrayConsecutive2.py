def makeArrayConsecutive2(sequence):
    if len(sequence)==1:
        return 0
    sequence.sort()
    res=0
    for i in range(1,len(sequence)):
        if sequence[i]!=(sequence[i-1]+1):
            res+=sequence[i]-sequence[i-1]-1
    return res
            
