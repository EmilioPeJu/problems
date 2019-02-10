#include <bits/stdc++.h>
#define INF 2147483648
#define MAX_DIM 21
using namespace std;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
long long sum[MAX_DIM][MAX_DIM][MAX_DIM];

// this is a 3D range sum version (max sum is O(n^6))
// however this could be improved if you use 2D range sum combined 
// with kadane algorithm applied to the third dimension (max sum would be O(n^5))
inline long long range_sum(int x1, int y1, int z1, int x2, int y2, int z2) {
	long long res = sum[x2][y2][z2];
	if (x1 > 0) res -= sum[x1-1][y2][z2];
	if (y1 > 0) res -= sum[x2][y1 - 1][z2];
	if (z1 > 0) res -= sum[x2][y2][z1 - 1];
	if (x1 > 0 && y1 > 0) res += sum[x1-1][y1-1][z2];
	if (x1 > 0 && z1 > 0) res += sum[x1-1][y2][z1-1];
	if (y1 > 0 && z1 > 0) res += sum[x2][y1-1][z1-1];
	if (x1 > 0 && y1 > 0 && z1 >0) res -= sum[x1-1][y1-1][z1-1];
	return res;
}

int main() {
	ios::sync_with_stdio(false);
	int tc;
	cin >> tc;
	while (tc--) {
		int a, b, c;
		cin >> a >> b >> c;
		for(int i=0;i<a;i++) {
			for(int j=0;j<b;j++) {
				for(int k=0;k<c;k++) {
					cin >> sum[i][j][k];
					if (i > 0)
						sum[i][j][k] += sum[i-1][j][k];
					if (j > 0)
						sum[i][j][k] += sum[i][j-1][k];
					if (k > 0)
						sum[i][j][k] += sum[i][j][k-1];
					if (i > 0 && j > 0)
						sum[i][j][k] -= sum[i-1][j-1][k];
					if (i > 0 && k > 0)
						sum[i][j][k] -= sum[i-1][j][k-1];
					if (j > 0 && k > 0)
						sum[i][j][k] -= sum[i][j-1][k-1];
					if (i>0 && j>0 && k>0)
						sum[i][j][k] += sum[i-1][j-1][k-1];
				}
			}
		}
		long long res = -INF;
		for(int x1=0;x1<a;x1++) {
			for(int y1=0;y1<b;y1++) {
				for(int z1=0;z1<c;z1++) {
					for(int x2=x1;x2<a;x2++) {
						for(int y2=y1;y2<b;y2++) {
							for(int z2=z1;z2<c;z2++) {
								long long current = range_sum(x1,y1,z1,x2,y2,z2);
								res = max(current, res);
							}
						}
					}
				}
			}
		}
		cout << res << endl;
		if(tc > 0) cout << endl;
	}
	return 0;
}
