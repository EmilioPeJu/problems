def differentSubstringsTrie(inputString):

    def addNode(lastVersion):
        lastVersion.append([0]*26)

    nodesCount = 1
    trie = []
    addNode(trie)

    for i in range(len(inputString)):
        currentNode = 0
        for j in range(i, len(inputString)):
            symbol = ord(inputString[j]) - ord('a')
            if not trie[currentNode][symbol]:
                addNode(trie)
                trie[currentNode][symbol] = nodesCount
                nodesCount += 1
            currentNode = trie[currentNode][symbol]

    return nodesCount - 1
