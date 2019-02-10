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
	if (n == 1) {
		cout << "No" << endl;
		return 0;
	}
	int acc = 0;
	for(int i=1;i<=n;i++) {
		acc += i;
	}
	bool itis = false;
	int val=0;
	for(int i=2;i<=n;i++) {
		if ((acc - i) % i == 0) {
			itis = true;
			val = i;
			break;
		}
	}
	if (itis) { 
		cout  << "Yes" << endl; 
		cout << "1 " << val << endl;
		cout << n - 1;
		for(int i=1;i<=n;i++) {
			if (i != val) cout << " " << i;
		}
		cout << endl;
	} else {    cout << "No" << endl; }

	return 0;
}
