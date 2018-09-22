def wordLadder(beginWord, endWord, wordList):
    
    def distance(word1, word2):
        if len(word1)!=len(word2):
            return -1
        res = 0
        for i in range(len(word1)):
            if word1[i]!=word2[i]:
                res+=1
        return res
    
    def get_children(current, wordList):
        for item in wordList:
            if distance(current, item)==1:
                yield item
                
    explored = set([beginWord])
    from collections import deque
    frontier = deque([(0, beginWord)])
    while len(frontier):
        dist, current = frontier.pop()
        if current==endWord:
            return dist+1
        for nei in get_children(current, wordList):
            if nei not in explored:
                frontier.appendleft((dist+1, nei))
                explored.add(nei)
    return 0