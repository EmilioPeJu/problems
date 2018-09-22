def formAdjListGraph(nodes,links):
    graph=[{} for _ in range(nodes)]
    for src,dst in links:
        graph[src][dst]=1
        graph[dst][src]=1
    return graph
def roadsBuilding(cities, roads):
    graph=formAdjListGraph(cities, roads)
    res=[]
    for i in range(len(graph)-1):
        for j in range(i+1,cities):
            if j not in graph[i]:
                res.append([i,j])
    return res