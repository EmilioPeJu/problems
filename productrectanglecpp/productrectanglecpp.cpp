#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
#define DIM 2001
long long table[DIM][DIM];
long long sum[DIM][DIM];
long long tmp[DIM];
long long A[DIM];
long long B[DIM];
int main() {
	ios::sync_with_stdio(false);
	int n, m;
	cin >> n >> m;
	for(int i=0;i<n;i++)
		cin >> A[i];
	for(int i=0;i<m;i++)
		cin >> B[i];
	for(int i=0;i<n;i++)
		for(int j=0;j<m;j++)
			table[i][j] = A[i]*B[j];
	long long max_sum = -1000000000000;
	for(int jstart=0;jstart<m;jstart++) {
		memset(tmp, 0, sizeof(tmp[0])*n);
		for(int jend=jstart;jend<m;jend++) {
			for(int i=0;i<n;i++) 
				tmp[i] += table[i][jend];
			// kadane 1D
			long long current_sum = 0;
			for(int i=0;i<n;i++) {
				current_sum += tmp[i];
				max_sum = max(max_sum, current_sum);
				if (current_sum < 0) current_sum = 0;
			}
		}
	}
	cout << max_sum << endl;
	return 0;
}
