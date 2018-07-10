#include <bits/stdc++.h>
#define MAX_DIM 101
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int tc;
	long long sum[MAX_DIM][MAX_DIM];
	cin >> tc;
	for(int test=0;test<tc;test++) {
		int N, M, K;
		cin >> N >> M >> K;
		for(int n=0;n<N;n++) {
			for(int m=0;m<M;m++) {
				cin >> sum[n][m];
				if (n > 0) sum[n][m] += sum[n-1][m];
				if (m > 0) sum[n][m] += sum[n][m-1];
				if (n > 0 && m > 0) sum[n][m] -= sum[n-1][m-1];
			}
		}
		int area = 0;
		long long area_sum = 10000000000LL;
		long long current_sum;
		long long current_area;
		for(int i=0;i<N;i++) {
			for(int j=0;j<M;j++) {
				for(int k=i;k<N;k++) {
					for(int l=j;l<M;l++) {
						current_area = (k-i + 1)*(l-j + 1);
						if (current_area < area) continue;
						current_sum = sum[k][l];
						if (i>0) current_sum -= sum[i-1][l];
						if (j>0) current_sum -= sum[k][j-1];
						if (i>0 && j>0) current_sum += sum[i-1][j-1];
						if (current_sum <= K && current_area >= area) {
							if (current_area > area || current_sum < area_sum) {
								area = current_area;
								area_sum = current_sum;
							}
						}
					}
				}
			}
		}
		cout << "Case #" << test + 1 << ": " << area << " " << (area==0?0:area_sum) << endl;
	}
	return 0;
}
