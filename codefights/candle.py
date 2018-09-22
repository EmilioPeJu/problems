def candles(candlesNumber, makeNew):
    candles=candlesNumber
    burn=0
    leftovers=0
    while True:
        burn+=candles
        leftovers+=candles
        candles=0
        if leftovers/makeNew==0:
            break
        else:
            candles=leftovers/makeNew
            leftovers=leftovers%makeNew
    return burn
