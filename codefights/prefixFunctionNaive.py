# PREFIX FUNCTION
# Given a string S of length n, prefix function produces an array P where for each i > 0, P[i] is the length of the longest substring ending with S[i] which is also a prefix of S, i.e. the maximal k such that S[j] = S[i - k + j + 1] for all 0 ≤ j < k. P[0] by definition equals to 0.
def prefixFunctionNaive(s):

    result = []

    for i in range(len(s)):
        result.append(0)
        for result[i] in range(i, -1, -1):
            matches = True
            for j in range(i - result[i] + 1, i + 1):
                if s[j] != s[j - i + result[i] - 1]:
                    matches = False
                    break
            if matches:
                break

    return result
