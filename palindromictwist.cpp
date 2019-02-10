#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

bool is_nice(char a, char b) {
	return (a+1) == (b+1) || (a-1) == (b+1) || (a+1) == (b-1) || (a-1) == (b-1);
}

int main() {
	ios::sync_with_stdio(false);
	char input[101];
	int tc;
	cin >> tc;
	while (tc--) {
		int n;
		cin >> n;
		cin >> input;
		bool not_nice = false;
		for(int i=0;i<n/2;i++) {
			if (!is_nice(input[i], input[n-1-i])) {
					not_nice = true;
					break;
			}
		}
		if (not_nice) cout << "NO" << endl;
		else 	      cout << "YES" << endl;

	}

	return 0;
}
