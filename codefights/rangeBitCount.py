def rangeBitCount(a, b):
    res=0
    for i in range(a,b+1):
        res+="{:b}".format(i).count("1")
    return res
