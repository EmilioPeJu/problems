def isPower(n):
    if n==1:
        return 1
    for i  in range(2,int(math.sqrt(n))+1):
        a=i*i
        while a<n:
            a*=i
        if a==n:
            return True
    return False