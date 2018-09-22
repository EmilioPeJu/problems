def bfsDistancesUnweightedGraph(matrix, startVertex):
    from collections import deque
    frontier=deque()
    frontier.append(startVertex)
    explored=set([startVertex])
    distances=[float("inf")]*len(matrix)
    distances[startVertex]=0
    while len(frontier):
        current=frontier.pop()
        for nei,state in enumerate(matrix[current]):
            if state and (nei not in explored):
                frontier.appendleft(nei)
                distances[nei]=distances[current]+1
                explored.add(nei)
    return distances
    