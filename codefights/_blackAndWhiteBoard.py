def blackAndWhiteBoard(h, w, queries):
    def isValid(x,y):
        return x>=0 and x<h and y>=0 and y<w
    cells=[[0]*w for _ in range(h)]
    res=[]
    ops={"<" : (0,-1), ">":(0,1), "v" :(1,0),"^":(-1,0)}
    for i in queries:
        x,y=[int(item) for item in i.split(" ")[1:]]
        if not isValid(x,y):
            res.append([-1,-1])
        if i[0]=="x":
            cells[x][y]=1
        else:
            newX=x
            newY=y
            direction=ops[i[0]]
            while isValid(newX+direction[0],newY+direction[1]) and  not cells[newX+direction[0]][newY+direction[1]]:
                newX+=direction[0]
                newY+=direction[1]
            if newY==y and newX==x:
                res.append([-1,-1])
            else:
                res.append([newX,newY])
    return res