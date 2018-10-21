#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int queries;
	cin >> queries;
	while (queries--) {
		int L, v, left, right;	
		cin >> L >> v >> left >> right;
		int res=L/v;
		if (left%v) left = left + (v - left%v);
		right = right - right%v + v - 1;
		int blind = (right - left + 1) / v;
		res -= blind;
		cout << res << endl;
	}
	return 0;
}
