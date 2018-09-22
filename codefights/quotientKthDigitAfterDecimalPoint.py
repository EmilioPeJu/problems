def quotientKthDigitAfterDecimalPoint(a, b, k):

    a = a % b
    while k > 1:
        a = (a * 10) % b
        k -= 1

    return (a * 10) / b