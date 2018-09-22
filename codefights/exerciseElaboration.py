def exerciseElaboration(p, n):
    def digits(num):
        dig = []
        while num:
            dig.append(num%10)
            num /= 10
        return dig
    if p==0:
        return 0
    elif n>2:
        return sum(digits(p*p))*2 + sum(digits(2*p*p))
    else:
        return sum(digits((p*10**(n+1)+p)**2))
        
