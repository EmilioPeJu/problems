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
	int res = 1;
	int a,b,c,d;
	int thomas_result;
	cin >> n;
	cin >> a >> b >> c >> d;
	thomas_result = a + b + c + d;
	for(int i=1;i<n;i++) {
		cin >> a >> b >> c >> d;
		if ((a+b+c+d) > thomas_result) res++;
	}
	cout << res << endl;

	return 0;
}
