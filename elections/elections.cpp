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
	int in;
	int maxin=1;
	int acc=0;
	cin >> n;
	for(int i=0;i<n;i++) {
		cin >> in;
		acc+=in;
		maxin=max(maxin, in);
	}
	int res=max(acc*2/n + 1, maxin);
	cout << res << endl;

	return 0;
}
