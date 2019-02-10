#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
class Uf {
	private:
		int n;
		vector<int> count;
		vector<int> parent;
	public:
		Uf(int n) : n(n) {
			for(int i=0;i<n;i++) {
				count.push_back(1);
				parent.push_back(i);
			}
		}

		int root(int a) {
			int current = a;
			while (current != parent[current])
				current=parent[current];
			return current;
		}

		int getMostFrequent(int a) {
			int result = -1;
			for(int i=0;i<(int)count.size();i++) {
				if (count[i] > count[result])
					result = i;
			}
			return result;
		}

		void connect(int a, int b) {
			int a_root = root(a);
			int b_root = root(b);
			if (a_root == b_root) return;
			if  (count[a_root] >= count[b_root]) {
				parent[b_root] = a_root;
				count[a_root] += count[b_root];
			} else {
				parent[a_root] = b_root;
				count[b_root] += count[a_root];
			}
		}

		int size(int a) {
			return count[a];
		}

		bool connected(int a, int b) {
			return root(a) == root(b);
		}
};

// I forced the use of a UF structure for
// solving this problem (as recommended for
// simplifying), but I am not sure if this was
// the right way.
// It is used as a set (the awaken) and as
// a counter
int main() {
#define tonum(x) (x - 'A')
	ios::sync_with_stdio(false);
	int n, nconnections;
	while (cin >> n >> nconnections) {
		Uf uf(27);
		vector<int> graph[27];
		vector<int> toConnect;
		char in;
		cin >> in;
		int root = tonum(in);
		cin >> in;
		uf.connect(root, tonum(in));
		cin >> in;
		uf.connect(root, tonum(in));
		for(int i=0;i<nconnections;i++) {
			char a, b;
			cin >> a >> b;
			graph[tonum(a)].push_back(tonum(b));
			graph[tonum(b)].push_back(tonum(a));
		}
		int years = 0;
		while (true) {
			for(int node=0;node<26;node++) {
				int cnt=0;
				//don't process waken nodes
				if (uf.connected(node, root)) continue;
				for(auto nei : graph[node]) {
					if (uf.connected(nei, root)) cnt++;	
					if (cnt >= 3) {
						toConnect.push_back(node);
						break;
					}
				}	
			}
			if (!toConnect.size()) break; 
			for(auto node : toConnect) uf.connect(node, root);
			toConnect.clear();
			years++;
		}
		if(uf.size(root) >= n) 
			cout << "WAKE UP IN, " << years << ", YEARS" << endl;
		else
			cout << "THIS BRAIN NEVER WAKES UP" << endl;
	}
	return 0;
}
