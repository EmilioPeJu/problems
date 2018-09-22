def create2DArray(lengths):
    res=[]
    for i in range (len(lengths)):
        res.append(list(range(lengths[i])))
    return res
