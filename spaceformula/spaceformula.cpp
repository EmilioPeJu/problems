#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
#define INF 100000001
int ranks[200001];
int points[200001];
int n, d;
int min_point = INF;
set<int> free_i;

int main() {
	ios::sync_with_stdio(false);
	cin >> n >> d;
	for(int i=0;i<n;i++)
		cin >> ranks[i];
	for(int i=n-1;i>=0;i--) { // non-decreasing order to use binary search
		cin >> points[i];
		min_point = min(min_point, points[i]);
	}
	for(int i=0;i<n;i++) {
		free_i.insert(i);
	}
	int pos=d - 1;
	int res=pos;
	int c_points=ranks[pos] + points[n-1];
	free_i.erase(n-1);
	for(int i=pos-1;i>=0;i--) {
		if (ranks[i] + min_point > c_points) break;
		int max_points = c_points - ranks[i];
		auto consider = upper_bound(points, points + n, max_points);
		int index = (int) (consider - points);
		index--; // lower or equal
		
		if (index == -1) continue;
		auto it = free_i.upper_bound(index);
		if (it == begin(free_i)) continue;
		it--;
		res--;
		free_i.erase(*it);
	}
	cout << res + 1 << endl;

	return 0;
}
