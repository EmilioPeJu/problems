def largestFullBinaryTree(parent):

    class Graph:

        def __init__(self, parent):
            self.maxBinTree = 1
            self.edges = []
            for i in range(len(parent)):
                self.edges.append([])
            for i in range(1, len(parent)):
                self.edges[parent[i]].append(i)

        def dfs(self, v):
            firstMax = -1
            secondMax = -1
            for u in self.edges[v]:
                curMax = self.dfs(u)
                if curMax > firstMax:
                    secondMax = firstMax
                    firstMax = curMax
                elif curMax > secondMax:
                    secondMax = curMax
            if secondMax == -1:
                return 1
            result = 1 + firstMax + secondMax
            if result > self.maxBinTree:
                self.maxBinTree = result
            return result

    g = Graph(parent)
    g.dfs(0)
    return g.maxBinTree
