def isInformationConsistent(evidences):
    for col in range(len(evidences[0])):
        result=0
        for row in range(len(evidences)):
            if result!=0 and evidences[row][col]!=0 and evidences[row][col]!=result:
                return False
            result=evidences[row][col]
    return True