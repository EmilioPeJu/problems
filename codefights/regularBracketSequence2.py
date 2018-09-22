def regularBracketSequence2(sequence):
    last=[]
    for i in sequence:
        if i=="(" or i=="[":
            last.append(i)
        if i==")":
            if last[-1]!="(":
                return False
            last.pop()
        if i=="]":
            if last[-1]!="[":
                return False
            last.pop()
    if len(last):
        return False
    return True
        
