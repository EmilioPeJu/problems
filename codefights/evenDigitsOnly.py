def evenDigitsOnly(n):
    while n:
        digit=n%10
        if digit%2==1:
            return False
        n/=10
    return True

