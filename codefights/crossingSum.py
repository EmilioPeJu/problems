def crossingSum(matrix, a, b):
    res=0
    for i in range(len(matrix[0])):
        if i!=b:
            res+=matrix[a][i]
    for i in range(len(matrix)):
        res+=matrix[i][b]
    return res