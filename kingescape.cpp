#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int n, ax, ay, bx, by, cx, cy;

int get_quadrant(int x, int y) {
	if (x < ax && y < ay) return 0;
	if (x < ax && y > ay) return 1;
	if (x > ax && y > ay) return 2;
	return 3;
}

int main() {
	ios::sync_with_stdio(false);
	cin >> n;
	cin >> ax >> ay;
	cin >> bx >> by;
	cin >> cx >> cy;
	if (get_quadrant(bx, by) == get_quadrant(cx, cy)) cout << "YES" << endl;
	else cout << "NO" << endl;
	return 0;
}
