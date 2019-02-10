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
	for(int a=1;a<n;a++) {
		for(int b=1;(a+b)<n;b++) {
			int c = n - a - b;
			if (a % 3 != 0 && b % 3 != 0 && c % 3 != 0) {
				cout << a << " " << b << " " << c << endl;
				return 0;
			}
		}
	}

	return 0;
}
