#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int in_val;
	int n;
	cin >> n;
	for(int i=0;i<n;i++) {
		cin >> in_val;
		if(i!=0) cout << " ";
		cout << in_val - (in_val%2==0);
	}
	cout << endl;

	return 0;
}
