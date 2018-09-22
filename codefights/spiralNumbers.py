def spiralNumbers(n):
    table=[[0]*n for _ in range(n)]
    table[0][0]=1
    lastx=0
    lasty=0
    dirs=[(0,1),(1,0),(0,-1),(-1,0)]
    dir_i=0
    def isValid(x,y):
        return (x>=0 and x<n and y>=0 and y<n)
    i=2
    while i<=n*n:
        if isValid(lastx+dirs[dir_i][0],lasty+dirs[dir_i][1]) and \
            table[lastx+dirs[dir_i][0]][lasty+dirs[dir_i][1]]==0:
                lastx=lastx+dirs[dir_i][0]
                lasty=lasty+dirs[dir_i][1]
                table[lastx][lasty]=i
                i+=1
        else:
            dir_i= (dir_i+1)%4
    return table