def verticesDegrees(matrix):
    deg=[]
    for i in range(len(matrix)):
        counter=sum([1 for item in matrix[i] if item])
        if matrix[i][i]:
            counter+=1
        deg.append(counter)
    return deg