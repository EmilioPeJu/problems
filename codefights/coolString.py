def coolString(inputString):
    if not inputString.isalpha():
        return False
    for i in range(1, len(inputString)):
        if not (int(inputString[i].islower()) ^ int(inputString[i-1].islower())):
            return False
    return True