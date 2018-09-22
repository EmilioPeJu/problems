#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int n, h;
int a, b;
int k;

long long dist(int t1, int f1, int t2, int f2) {
	if (t1 == t2) return abs(f2 - f1);
	if (f1<=b && f1 >=a) return abs(t2-t1) + abs(f2 - f1);
	long long dist1 = abs(f1-a) + abs(f2-a);
	long long dist2 = abs(f1 - b) + abs(f2 - b);
	return min(dist1, dist2) + abs(t2 - t1);
}

int main() {
	int t1, f1, t2, f2;
	ios::sync_with_stdio(false);
	cin >> n >> h;
	cin >> a >> b;
	cin >> k;
	for(int i=0;i<k;i++) {
		cin >> t1 >> f1 >> t2 >> f2;
		cout << dist(t1, f1, t2, f2) << endl;
	}

	return 0;
}
