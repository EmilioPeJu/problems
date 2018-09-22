def treeLCA(parent, v1, v2):
    def path_to_parent(v):
        path=[]
        while parent[v]!=v:
            path.append(v)
            v=parent[v]
        path.append(v)
        return path
    path=path_to_parent(v1)
    visited=set(path)
    v=v2
    while v not in visited:
        v=parent[v]
    return v