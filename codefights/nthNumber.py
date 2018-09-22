def nthNumber(s, n):
    import re
    res = re.findall("\d+",s)
    return str(int(res[n-1]))
