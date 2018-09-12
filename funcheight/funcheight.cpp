#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	long long k, n, res;
	cin >> n >> k;
	res = k / n + (k%n != 0);
	cout << res << endl;
	return 0;
}
