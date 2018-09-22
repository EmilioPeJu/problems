def sameDigitNumber(n):
    first=n%10
    while n:
        current=n%10
        if current!=first:
            return False
        first=current
        n/=10
    return True
