def alphabetSubstring(s):
    alph=[chr(i) for i in range(ord('a'),ord('z')+1)]
    alph="".join(alph)
    return s in alph
