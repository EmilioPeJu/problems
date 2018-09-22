def subtractionGame(n):

    def checkFirstWins(n, move):
        if n <= 0:
            return True
        for i in range(1, move + 1):
            if not checkFirstWins(n - i, move + 1):
                return True
        return False

    return  checkFirstWins(n,1) 