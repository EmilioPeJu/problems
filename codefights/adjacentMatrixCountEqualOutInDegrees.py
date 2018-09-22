def newRoadSystem(roadRegister):
    for i in range(len(roadRegister[0])):
        col=0
        row=0
        for k in range(len(roadRegister[0])):
            if roadRegister[i][k]:
                row+=1
            if roadRegister[k][i]:
                col+=1
        if row!=col:
            return False
    return True