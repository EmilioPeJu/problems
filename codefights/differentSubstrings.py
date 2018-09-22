def differentSubstrings(inputString):
    ans=0
    all_str=set()
    for i in range(1,len(inputString)+1):
        for j in range(0,len(inputString)-i+1):
            substr=inputString[j:j+i]
            if substr not in all_str:
                all_str.add(substr)
                ans+=1
    return ans