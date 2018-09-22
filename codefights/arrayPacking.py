def arrayPacking(a):
    res=0
    i=0
    for item in a:
        res|=(item<<(8*i)) 
        i+=1
    return res
