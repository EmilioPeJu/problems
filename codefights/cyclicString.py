def cyclicString(s):
    for i in range(len(s)+1):
        for j in range(0,len(s)-i):
            if s in s[j:j+i]*15:
                return i
    return len(s)
