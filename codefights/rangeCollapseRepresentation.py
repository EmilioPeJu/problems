def rangeCollapseRepresentations(a):

    ans = 1
    for i in range(1, len(a)):
        if a[i] > a[i - 1] + 1:
            ans *= 2

    return ans