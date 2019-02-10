#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int n, m;
        char table[116][116];
	cin >> n >> m;
	int firstbi=0, firstbj=0;
	for(int i=0;i<n;i++) {
		for(int j=0;j<m;j++) {
			cin >> table[i][j];
			if (table[i][j] == 'B' && firstbi == 0) {
				firstbi = i; firstbj = j;
			}
		}
	}
	int x, resi, resj;
	x = firstbi;
	while (x < n && table[x][firstbj] == 'B') x++;
	x--;
	resi = (firstbi + x) / 2;
	x = firstbj;
	while (x < m && table[firstbi][x] == 'B') x++;
	x--;
	resj = (firstbj + x) / 2;
	cout << resi + 1 << " " << resj + 1 << endl;
	
	return 0;
}
