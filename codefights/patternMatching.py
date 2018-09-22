def patternMatching(inputStr, pattern):

    dp = []
    for i in range(len(inputStr) + 1):
        line = []
        for j in range(len(pattern) + 1):
            line.append(False)
        dp.append(line)

    dp[0][0] = True
    for i in range(len(inputStr) + 1):
        for j in range(len(pattern)):
            if not dp[i][j]:
                continue
            if (i < len(inputStr)
            and (inputStr[i] == pattern[j] or pattern[j] == '?')):
                dp[i + 1][j + 1] = True
            if pattern[j] == '*':
                for k in range(len(inputStr) - i + 1):
                    dp[i + k][j + 1] = True

    return dp[len(inputStr)][len(pattern)]
