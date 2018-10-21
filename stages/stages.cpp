#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int n, k;
	cin >> n >> k;
	char input[60];
	cin >> input;
	sort(input, input+n);
	int step=1;
	int last=0;
	int current=1;
	int res = input[last] - 'a' + 1;
	while (current < n && step < k) {
		if ((input[current] - input[last]) < 2) { current++; continue; }
		last = current++;
		res += input[last] - 'a' + 1;
		step++;
	}
	if (step >= k) cout << res << endl;
	else cout << -1 << endl;

	return 0;
}
