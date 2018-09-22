def whoseMove(lastPlayer, win):
    players=["black", "white"]
    return players[players.index(lastPlayer)^int(not win)]