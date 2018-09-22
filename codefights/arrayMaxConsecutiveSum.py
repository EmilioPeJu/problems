def arrayMaxConsecutiveSum(inputArray, k):
    max_res = sum(inputArray[0:k])
    res = max_res
    for i in range(k, len(inputArray)):
        res = res - inputArray[i-k] + inputArray[i]
        if res > max_res:
            max_res = res
    return max_res

