def divisorsPairs(sequence):
    res = 0
    for ifirst in range(len(sequence) - 1):
        first = sequence[ifirst]
        if first==0:
            continue
        for isecond in range(ifirst + 1, len(sequence)):
            second = sequence[isecond]
            if second == 0:
                continue
            if first%second == 0 or second%first == 0:
                res+=1
    return res