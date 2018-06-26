#include <bits/stdc++.h>
#define MAXV 201
using namespace std;
typedef vector<int> adjlist;
adjlist graph[MAXV];
bool visited[MAXV];
int color[MAXV];
bool bicolor = true;

void dfs_bicolor(int node) {
	if (visited[node] || !bicolor) return;
	visited[node] = true;
	int expected = color[node] == 1 ? 2 : 1;
	for(size_t i=0; i<graph[node].size();i++) {
		int adj = graph[node][i];
		if (color[adj] == 0) {
			color[adj] = expected;
		} else if (color[adj] != expected) {
			bicolor = false;
			return;
		} 
		dfs_bicolor(adj);
	}
}

int main() {
	int nodes, edges;
	while (true) {
		cin >> nodes;
		if(nodes == 0) break;
		cin >> edges;
		for(int i=0;i<nodes;i++)
			graph[i].clear();
		for(int i=0;i<edges;i++) {
			int a, b;
			cin >> a >> b;
			graph[a].push_back(b);	
			graph[b].push_back(a);	
		}
		bicolor = true;
		memset(visited, 0, sizeof(bool)*nodes);
		memset(color, 0, sizeof(int)*nodes);
		for(int i=0;i<nodes;i++) {
			if (graph[i].size()) {
				color[i] = 1;
				dfs_bicolor(i);
				break;
			}
		}
		if (bicolor) cout << "BICOLORABLE." << endl;
		else	     cout << "NOT BICOLORABLE." << endl;
	}

	return 0;
}
