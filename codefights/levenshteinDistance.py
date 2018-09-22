def levenshteinDistance(string1, string2):

    len1 = len(string1)
    len2 = len(string2)
    dp = []
    for i in range(len1 + 1):
        dp.append([0] * (len2 + 1))
    for i in range(len1 + 1):
        dp[i][0] = i
    for j in range(len2 + 1):
        dp[0][j] = j

    for i in range(1, len1 + 1):
        for j in range(1, len2 + 1):
            if string1[i - 1] == string2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1]
                continue
            dp[i][j] = min(dp[i - 1][j - 1],
                    min(dp[i][j - 1], dp[i - 1][j]))+1

    return dp[len1][len2]