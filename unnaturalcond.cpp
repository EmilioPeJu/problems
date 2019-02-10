#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int n, m;
	int n_dig;
	cin >> n >> m;
	n_dig = n / 5 + (n%5!=0);
	for(int times=0;times<2;times++) {
		for(int i=0;i<n_dig;i++) {
			cout << 5;
			}
		cout << endl;
	}
	

	return 0;
}
