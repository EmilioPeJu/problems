def differentDigitsNumberSearch(inputArray):
    for i in inputArray:
        c_set=set()
        isValid=True
        for c in str(i):
            if c in c_set:
                isValid=False
                break
            c_set.add(c)
        if isValid:
            return i
    return -1
            