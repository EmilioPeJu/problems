#include <bits/stdc++.h>
#define MAX_GARMENTS 21
#define MAX_MODELS 21
#define MAX_MONEY 401
#define INF 1000000000
using namespace std;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int garments[MAX_GARMENTS][MAX_MODELS];
int dp[MAX_MONEY][MAX_GARMENTS];
int n_money;
int n_garments;

// state: money I have, garment i deciding
int _topdown_garments(int money, int garment) {
	if (money < 0) return INF;	
	if (dp[money][garment] != -1) return dp[money][garment];
	if (garment == n_garments) return money;
	int res = INF;
	for(int i=1;i <= garments[garment][0];i++) {
		int val = _topdown_garments(money - garments[garment][i], garment + 1);
		res = min(val, res);
	}
	dp[money][garment] = res;
	return res;
}

int topdown_garments(int money, int garment) {
	memset(dp, -1, sizeof(dp));
	return _topdown_garments(money, garment);
}

int main() {
	ios::sync_with_stdio(false);
	int tc;
	cin >> tc;
	while (tc--) { 
		cin >> n_money;
		cin >> n_garments;
		for(int i=0;i < n_garments;i++) {
			cin >> garments[i][0];
			for(int j=1;j<=garments[i][0];j++)
				cin >> garments[i][j];
		}
		int money_left = topdown_garments(n_money, 0);
		int result = n_money - money_left;
		if (money_left == INF) cout << "no solution" << endl;
		else cout << result << endl;
	}

	return 0;
}
