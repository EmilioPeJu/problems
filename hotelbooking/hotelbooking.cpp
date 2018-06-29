#include <bits/stdc++.h>
#define MAXV 10001
#define INF 2000000000
#define MAX_DIST 600
using namespace std;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
vii graph[MAXV]; // vector of links (first is dst node, second is time to reach it)
bool hotels[MAXV];
const int start = 1;
int n_nodes;
int n_hotels;
int n_links;

bool min_hotels_faster() {
	bool visited[MAXV] = {0};
	priority_queue<ii, vii, greater<ii>> frontier;
	frontier.push(make_pair(0, start));
	while (frontier.size()) {
		ii current = frontier.top();
		frontier.pop();
		int dist = current.first;
		int node = current.second;
		if (visited[node]) continue;
		if (node == n_nodes) return true;
		visited[node] = true;
		for (auto adj: graph[node]) {
			int adj_node = adj.first;
			int adj_weight = adj.second;
			int adj_dist = (hotels[node]?0:dist) + adj_weight;
			frontier.push(make_pair(adj_dist, adj_node));
		}
	}
	return false;
}

void min_hotels_calc() {
	bool visited[MAXV][MAX_DIST + 1];
	for(int i=1; i<=n_nodes; i++) {
		memset(visited[i], 0, sizeof(visited[i])*sizeof(bool));
	}
	priority_queue <iii, viii, greater<iii > > frontier;
	frontier.push(make_tuple(0, 0, start));
	while (frontier.size()) {
		iii current = frontier.top();
		frontier.pop();
		int node = get<2>(current);
		int node_hotels = get<0>(current);
		int node_dist_fh = get<1>(current);
		if (visited[node][node_dist_fh]) continue;
		if (node == n_nodes) {
			cout << node_hotels << endl;
			return;
		}
		visited[node][node_dist_fh] = true;
		for (auto adj: graph[node]) {
			int adj_node = adj.first;
			int adj_weight = adj.second;
			int new_adj_dist_fh = node_dist_fh + adj_weight;
			//if (visited[adj_node][new_adj_dist_fh]) continue;
			if (new_adj_dist_fh <= MAX_DIST) {
				// expand only if we don't exceed the 10 hours travel
				frontier.push(make_tuple(node_hotels, new_adj_dist_fh, adj_node));
			}
			if (hotels[node] && adj_weight <= MAX_DIST) {
				new_adj_dist_fh = adj_weight;
				frontier.push(make_tuple(node_hotels + 1, new_adj_dist_fh, adj_node));
			}
		}
	}
	cout << "-1" << endl;
}

int main() {
	ios::sync_with_stdio(false);
	while (true) {
		cin >> n_nodes;
		if (n_nodes == 0) break;
		for(int i=1; i <= n_nodes; i++) { 
			graph[i].clear();
		}
		memset(hotels, 0, (n_nodes + 1)*sizeof(bool));
		cin >> n_hotels;
		for(int i=0;i<n_hotels;i++) {
			int input;
			cin >> input;
			hotels[input] = true;
		}
		cin >> n_links;
		for(int i=0;i<n_links;i++) {
			int a, b, t;
			cin >> a >> b >> t;	
			graph[a].push_back(make_pair(b, t));
			graph[b].push_back(make_pair(a, t));
		}
		if (!min_hotels_faster())
			cout << "-1" << endl;
		else
			min_hotels_calc();
	}

	return 0;
}
