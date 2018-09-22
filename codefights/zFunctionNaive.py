def zFunctionNaive(s):

    result = []

    for i in range(len(s)):
        result.append(0)
        for j in range(i, len(s)):
            if s[j] == s[result[i]]:
                result[i] += 1
            else:
                break

    return result