#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int n, a=1, b=1;
	char result[1001];
	cin >> n;
	memset(result, 'o', n);
	result[n] = '\0';
	while (b <= n) {
		result[b-1] = 'O';
		swap(a, b);
		b = a + b;
	}
		
	cout << result << endl;
	return 0;
}
