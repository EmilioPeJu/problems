def bfsDistancesUnweightedGraph(matrix, startVertex):
    distances=[-1]*len(matrix)
    distances[startVertex]=0
    visited=set()
    import collections
    frontier=collections.deque()
    frontier.appendleft(startVertex)
    while len(frontier):
        node=frontier.pop()
        for nei,state in enumerate(matrix[node][:]):
            if state and distances[nei]==-1: # check if explorer with distances, in bfs first time it's disvered is the shortest
                frontier.appendleft(nei)
                distances[nei]=1+distances[node]
    return distances
    