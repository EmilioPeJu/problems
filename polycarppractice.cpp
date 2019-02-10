#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int arr[2001];
int arr2[2001];

int main() {
	ios::sync_with_stdio(false);
	int n, k;
	multiset<int> high_vals;
	cin >> n >> k;
	for(int i=0;i<n;i++) {
		cin >> arr[i];
		arr2[i] = arr[i];
	}
	sort(arr2, arr2 + n);
	int res=0;
	for(int i=n-1;i>=(n-k);i--) {
		res += arr2[i];	
		high_vals.insert(arr2[i]);
	}
	cout << res << endl;
	int index=0;
	int share=0;
	for(int i=0;i<k-1;i++) {
		share=0;
		multiset<int>::iterator it;
		while ((it=high_vals.find(arr[index++])) == high_vals.end()) {
			share++;
		}
		high_vals.erase(it);
		cout << share + 1 << " ";
	}
	cout << n - index << endl;
	return 0;
}
