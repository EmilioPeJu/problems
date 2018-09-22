def mirrorBits(a):
    res=0
    while a>0:
        res<<=1
        res=res|(a%2)
        a/=2
    return res
