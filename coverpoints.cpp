#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int res = 0;
	int a,b;
	int n;
	cin >> n;
	for(int i=0;i<n;i++) {
		cin >> a >> b;
		res = max(res, a+b);
	}
	cout << res << endl;

	return 0;
}
