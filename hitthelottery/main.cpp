#include <bits/stdc++.h>
using namespace std;

int main() {
	int n;
	int res=0;
	int coins[] = {1, 5, 10, 20, 100};
	int n_coins = sizeof(coins)/sizeof(int);
	cin >> n;
	for (int i=n_coins-1;i>=0;i--) {
		res+=n / coins[i];
		n = n % coins[i];
	}
	cout << res << endl;
	return 0;
}
