def passwordCheck(inputString):
    digit=False
    upper=False
    lower=False
    for i in inputString:
        if i.isdigit():
            digit=True
        if i.isupper():
            upper=True
        if i.islower():
            lower=True
    return all((digit,upper,lower,len(inputString)>=5))
