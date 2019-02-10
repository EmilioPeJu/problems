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
	int N;
	long long sum[MAX_DIM][MAX_DIM];
	while (true) {
		cin >> N;
		if (cin.eof()) break;
		for(int i=0;i<N;i++) {
			for(int j=0;j<N;j++) {
				cin >> sum[i][j];
				if (i > 0)
					sum[i][j] += sum[i-1][j];
				if (j > 0)
					sum[i][j] += sum[i][j-1];
				if (i > 0 && j > 0)
					sum[i][j] -= sum[i-1][j-1];
			}
		}
		long long res = -127;
		long long current_sum;
		for(int i=0;i<N;i++)
			for(int j=0;j<N;j++)
				for(int k=i;k<N;k++)
					for(int l=j;l<N;l++) {
						current_sum = sum[k][l];
						if (i > 0)
							current_sum -= sum[i-1][l];
						if (j > 0)
							current_sum -= sum[k][j-1];
						if (i > 0 && j > 0)
							current_sum += sum[i-1][j-1];
						res = max(res, current_sum);
					}	
		cout << res << endl;
	}
	return 0;
}
