def nextSecond(currentTime):
    nextTime=list(currentTime)
    nextTime[2]+=1
    if nextTime[2]>=60:
        nextTime[2]=0
        nextTime[1]+=1
    if nextTime[1]>=60:
        nextTime[1]=0
        nextTime[0]+=1
    if nextTime[0]>=24:
        nextTime[0]=0
    return nextTime