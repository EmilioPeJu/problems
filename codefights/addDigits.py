def addDigits(a, b, n):
    def lengthen(num, step, final, max_val):
        for i in range(9,-1,-1):
            res = num*10+i
            if res%b==0:
                if res>max_val:
                    max_val = res
                if step==final:
                    return res
                else:
                    return lengthen(res, step+1, final, max_val)
        return max_val
    return str(lengthen(a, 1, n, a))