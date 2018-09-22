def mergeKArrays(arrays):
    res=[]
    for i in arrays:
        res.extend(i)
    res.sort()
    return res
