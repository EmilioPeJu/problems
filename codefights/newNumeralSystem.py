def newNumeralSystem(number):
    base="ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    assert len(base)==26
    result=[]
    number=base.index(number)
    for i in range(number/2+1):
        result.append(base[i]+ " + " + base[number-i])
    return result
