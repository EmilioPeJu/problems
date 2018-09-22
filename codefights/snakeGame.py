def snakeGame(gameBoard, commands):
    def parseDirection(symbol):
        if symbol == '<':
            return [0, -1]
        if symbol == '>':
            return [0, 1]
        if symbol == '^':
            return [-1, 0]
        if symbol == 'v':
            return [1, 0]

    def findSnakeHead():
        for i in range(len(gameBoard)):
            for j in range(len(gameBoard[0])):
                if gameBoard[i][j] != '*' and gameBoard[i][j] != '.':
                    startX = i
                    startY = j
                    direction = parseDirection(gameBoard[startX][startY])
                    return (direction, startX, startY)

    def findSnakeTail(x, y):
        gameBoard[x][y] = '.'
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                if (dx * dy == 0 and dx + dy != 0 and x + dx >= 0 and x + dx < len(gameBoard)
                 and y + dy >= 0 and y + dy < len(gameBoard[0])):
                    if gameBoard[x + dx][y + dy] == '*':
                        findSnakeTail(x + dx, y + dy)
        queue.append([x, y])

    def drawSnake(bodyCharacter):
        for k in range(len(queue) - 1, len(queue) - snakeLength - 1, -1):
            gameBoard[queue[k][0]][queue[k][1]] = bodyCharacter

    startX = -1
    startY = -1
    queue = []
    curDir = []
    snakeLength = -1
    directionChars = [[' ', '^', ' '],
                      ['<', ' ', '>'],
                      [' ', 'v', ' ']]

    curDir, startX, startY = findSnakeHead()
    findSnakeTail(startX, startY)
    snakeLength = len(queue)
    for i in range(len(commands)):
        x = queue[-1][0]
        y = queue[-1][1]
        if commands[i] == 'L':
            curDir = [-curDir[1],curDir[0]]
            continue
        if commands[i] == 'R':
            curDir = [curDir[1], -curDir[0]]
            continue
        x += curDir[0]
        y += curDir[1]
        if x < 0 or y < 0 or x >= len(gameBoard) or y >= len(gameBoard[0]):
            drawSnake('X')
            return gameBoard
        for j in range(len(queue) - 1, len(queue) - snakeLength, -1):
            if queue[j][0] == x and queue[j][1] == y:
                drawSnake('X')
                return gameBoard
        queue.append([x, y])

    drawSnake('*')

    startX = queue[-1][0]
    startY = queue[-1][1]
    gameBoard[startX][startY] = directionChars[curDir[0] + 1][curDir[1] + 1]

    return gameBoard