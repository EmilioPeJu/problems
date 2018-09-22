def depositProfit(deposit, rate, threshold):
    current = deposit
    i = 0
    while True:
        if current >= threshold:
            return i
        current = current * (1.0 + float(rate)/100)
        i += 1