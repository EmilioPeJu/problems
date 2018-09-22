def maxNumberOfDivisorsPermutation2(n):

    class Helper:

        def __init__(self):
            self.digits = [0] * 10
            self.bestNumber = -1
            self.maxDivisors = 0

        def generatePermutation(self, curValue):
            lastStep = True
            for i in xrange(9, -1, -1):
                if self.digits[i] > 0:
                    self.digits[i] -= 1
                    self.generatePermutation(curValue * 10 + i)
                    self.digits[i] += 1
                    lastStep = False
            if lastStep:
                cntDivisors = 0
                for i in xrange(1, curValue + 1):
                    if curValue % i == 0:
                        cntDivisors += 1
                if cntDivisors > self.maxDivisors:
                    self.bestNumber = curValue
                    self.maxDivisors = cntDivisors

    h = Helper()

    while n > 0:
        h.digits[n % 10] += 1
        n /= 10

    h.generatePermutation(0)

    return h.bestNumber