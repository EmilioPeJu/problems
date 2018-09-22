def buildPalindrome(st):
    part1 = [ i for i in st]
    part2 = []
    def isPal(item):
        for i in range(len(item)/2):
            if item[i]!=item[-1-i]:
                return False
        return True
    i = 0
    while not isPal(''.join(part1 + list(reversed(part2)))):
        part2.append(part1[i])
        i += 1
    return ''.join(part1+list(reversed(part2)))
