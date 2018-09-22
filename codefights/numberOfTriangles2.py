def numberOfTriangles2(sticks):

    ans = 0
    for i in range(0, len(sticks) - 2):
        for j in range(i + 1, len(sticks) - 1):
            mx = sticks[i] + sticks[j]
            l = j
            r =  len(sticks)
            while r - l > 1:
                m = (l + r) / 2
                if sticks[m] >= mx:
                    r = m
                else:
                    l = m
            ans += r - j - 1

    return ans