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
	int last, w, h;
	cin >> w >> h;
	last = max(w, h);
	bool is_good = true;
	for(int i=1;i<n;i++) {
		cin >> w >> h;
		if (w > h)
			swap(w, h);
		if (w > last) {
			is_good = false;
			break;
		}
		if (h <= last)
			last = h;
		else
			last = w;
	}
	if (is_good)
		cout << "YES" << endl;
	else
		cout << "NO" << endl;
	return 0;
}
