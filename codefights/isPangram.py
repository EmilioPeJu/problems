def isPangram(sentence):
    x = [item.lower() for item in sentence if  item.isalpha()]
    x = list(set(x))
    x.sort()
    x = ''.join(x)
    print(x)
    return x=="abcdefghijklmnopqrstuvwxyz"