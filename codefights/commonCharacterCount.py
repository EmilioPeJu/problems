def commonCharacterCount(s1, s2):
    letters_s1 = [0]*26
    letters_s2 = [0]*26
    for i in s1:
        letters_s1[ord(i)-ord('a')]+=1
    for i in s2:
        letters_s2[ord(i)-ord('a')]+=1
    res = 0
    for i in range(26):
        res+=min(letters_s1[i], letters_s2[i])
    return res