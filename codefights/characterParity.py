def characterParity(symbol):
    if not symbol.isdigit():
        return "not a digit"
    return ["even","odd"][int(symbol)%2]
