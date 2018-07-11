#include <bits/stdc++.h>
#define MAX_DEPTH 9
#define MAX_DIST 1000 
#define INF 2000000000
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	// desc: shortest path in a DAG
	// state: i=period h=depth from 9 miles
	// transition: fp(i, h) = min(fp(i-1, h) + 30,
	// 			 fp(i-1, h+1) + 20,
	// 			 fp(i-1, h-1) + 60) - wind[i-1][h])
	// base: fp(0,9) = 0;
	// sol: fp(X,9)  
	int tc;
	int wind[MAX_DIST+1][MAX_DEPTH+1];
	int dp[MAX_DIST+2][MAX_DEPTH+1];
	cin >> tc;
	while(tc--) {
		int X;
		cin >> X;
		X /= 100;
		for(int i=0;i<=MAX_DEPTH;i++) {
			for(int j=0;j<X;j++) {
				cin >> wind[j][i];
			}
		}
		for(int i=0;i<=X;i++) {
			for(int j=0;j<=MAX_DEPTH;j++) {
				dp[i][j] = INF;
			}
		}
		dp[0][MAX_DEPTH] = 0;
		for(int i=0;i<X;i++) {
			for(int j=0;j<=MAX_DEPTH;j++) {
				int current = dp[i][j] - wind[i][j];
				int right = current + 30;
				dp[i+1][j] = min(dp[i+1][j], right);
				if (j > 0) {
					int up = current + 60;
					dp[i+1][j-1] = min(dp[i+1][j-1], up);
				}
				if (j < MAX_DEPTH) {
					int down = current + 20;
					dp[i+1][j+1] = min(dp[i+1][j+1], down);
				}
			}
		}
		cout << dp[X][MAX_DEPTH] << endl << endl;
	}
	return 0;
}
