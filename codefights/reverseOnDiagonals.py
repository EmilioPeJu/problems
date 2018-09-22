def reverseOnDiagonals(matrix):
    for i in range(len(matrix)/2):
        matrix[i][i],matrix[len(matrix)-i-1][len(matrix)-i-1]=matrix[len(matrix)-i-1][len(matrix)-i-1],matrix[i][i]
        matrix[len(matrix)-i-1][i],matrix[i][len(matrix)-i-1]=matrix[i][len(matrix)-i-1],matrix[len(matrix)-i-1][i]
    return matrix