def checkSameElementExistence(arr1, arr2):
    arr1=set(arr1)
    arr2=set(arr2)
    return len(arr1.intersection(arr2))