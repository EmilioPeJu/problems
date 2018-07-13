#include <bits/stdc++.h>
#define MAX_N 100000
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int arr[MAX_N];
	int ordered_arr[MAX_N];
	bool visited[MAX_N];
	int n;
	cin >> n;
	for(int i=0;i<n;i++) {
		cin >> arr[i];
		ordered_arr[i] = arr[i];
		visited[i] = false;
	}
	sort(ordered_arr, ordered_arr + n);
	int ans = 0;
	for(int i=0;i<n;i++) {
		auto it = lower_bound(ordered_arr, ordered_arr + n, arr[i]);
		while (*it >= arr[i] && it >= ordered_arr  || visited[it - ordered_arr])
			it--;
		if(it>=ordered_arr) {
			visited[it - ordered_arr] = true;
			ans++;
		}
	}
	cout << ans << endl;
	
	return 0;
}
