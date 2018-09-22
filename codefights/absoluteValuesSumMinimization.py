def absoluteValuesSumMinimization(A):
    res=float("inf")
    res_i=-1
    for i in A:
        aux_res=0
        for j in range(len(A)):
            aux_res+=abs(A[j]-i)
        if aux_res<res:
            res=aux_res
            res_i=i
    return res_i
        