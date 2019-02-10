#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int n, L, a, t, l;
	cin >> n >> L >> a;
	long long res = 0;
	int current = 0;
	for(int i=0;i<n;i++) {
		cin  >> t >> l;
		res += (t - current) / a;
		current = t + l;
	}
	res += (L - current) / a;
	cout << res << endl;

	return 0;
}
