#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int cnt[26]={0};
	int k, n;
	string input;
	cin >> n >> k;
	cin >> input;
	for(auto chr: input)
		cnt[chr - 'A']++;
	int result = 100000;
	for(int i=0;i<k;i++) {
		result = min(result, cnt[i]*k);
		if(cnt[i] == 0)
			break;
	}
	cout << result << endl;

	return 0;
}
