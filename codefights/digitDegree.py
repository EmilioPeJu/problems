def digitDegree(n):
    res=0
    while n>9:
        aux=n
        part_res=0
        while aux:
            part_res+=aux%10
            aux/=10
        n=part_res
        res+=1
    return res