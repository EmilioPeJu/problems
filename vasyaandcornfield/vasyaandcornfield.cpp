#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int n, d, m, x, y;
#define line1(x) (-x + d)
#define line2(x) (x - d)
#define line3(x) (-x +2*n-d)
#define line4(x) (x + d)

int main() {
	ios::sync_with_stdio(false);
	cin >> n >> d;
	cin >> m;
	for(int i=0;i<m;i++) {
		cin >> x >> y;
		if (line1(x) <= y && line2(x) <= y && line3(x) >= y && line4(x) >= y)
			cout << "YES" << endl;
		else
			cout << "NO" << endl;
	}

	return 0;
}
