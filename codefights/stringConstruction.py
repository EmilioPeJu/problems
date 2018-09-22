def stringsConstruction(A, B):
    def count(x):
        counter={}
        for i in x:
            counter.setdefault(i,0)
            counter[i]+=1
        return counter
    a_count=count(A)
    b_count=count(B)
    ans=float("inf")
    for key in a_count:
        if key not in b_count:
            return 0
        ans=min((ans,b_count[key]/a_count[key]))
    return ans

