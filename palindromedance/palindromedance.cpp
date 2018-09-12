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
	int dancers[21];
	int a[3];
	cin >> n >> a[0] >> a[1];
	a[2] = min(a[0], a[1]);
	for(int i=0;i<n;i++)
		cin >> dancers[i];
	long long cost = 0;
	for(int i=0;i<n/2;i++) {
		int xordance = dancers[i] ^  dancers[n - i - 1];
		if (xordance == 1) {
			cout << -1 << endl;
			return 0;
		} else if (xordance & 2) { 
			cost += a[xordance & 1];
		} else if (dancers[i] == 2) {
			cost += a[2]*2;
		}
	}
	if (n%2==1 && dancers[n/2] == 2) cost += a[2]; 
	cout << cost << endl;

	return 0;
}
