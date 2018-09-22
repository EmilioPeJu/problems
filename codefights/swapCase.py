def swapCase(text):
    res=[]
    for i in text:
        if ord(i)>=ord('a') and ord(i)<=ord('z'):
            res.append(i.upper())
        elif ord(i)>=ord('A') and ord(i)<=ord('Z'):
            res.append(i.lower())
        else:
            res.append(i)
    return "".join(res)