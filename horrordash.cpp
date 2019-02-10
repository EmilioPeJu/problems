#include <bits/stdc++.h>
using namespace std;

int main() {
	int T;
	cin >> T;
	for(int tc=0;tc<T;tc++) {
		int n, max_v, v;
		max_v=0;
		cin >> n;
		for (int i=0;i<n;i++) {
			cin >> v;
			max_v=max(max_v, v);
		}
		cout << "Case " << tc + 1 << ": " << max_v << endl;
	}

	return 0;
}
