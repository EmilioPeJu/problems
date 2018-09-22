def houseOfCats(legs):
    if legs==0:
        return [0]
    persons=0
    ans=[]
    while persons*2<=legs:
        if (legs-persons*2)%4==0:
            ans.append(persons)
        persons+=1
    return ans
