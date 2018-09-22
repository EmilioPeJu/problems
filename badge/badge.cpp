#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int n;
	cin >> n;
	int arr[1005];
	bool visited[1005];
	for(int i=1;i<=n;i++) {
		cin >> arr[i];
	}
	for(int i=1;i<=n;i++) {
		memset(visited, 0, n+1);
		int current = i;
		while (!visited[current]) {
			visited[current] = true;
			current = arr[current];
		}
		cout << current << " ";
	}
	cout << endl;

	return 0;
}
