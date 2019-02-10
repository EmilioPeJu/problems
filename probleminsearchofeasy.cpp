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
	int result = 0;
	for(int i=0;i<n;i++) {
		int input;
		cin >> input;
		result |= input;
	}
	cout << (result?"HARD":"EASY") << endl;

	return 0;
}
