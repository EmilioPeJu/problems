#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
#define is_good(a,b) (table[a][b]=='G')
#define is_valid(a,b) (a >= 0 && b >= 0 && a < n && b < m && table[a][b] == 'G')
char table[20][20];
int n, m;

void set_cross(int i, int j, int hop, char val) {
		table[i][j] = val;
		while(hop>0) {
			table[i + hop][j] = val;
			table[i - hop][j] = val;
			table[i][j + hop] = val;
			table[i][j - hop] = val;
			hop--;
		}
}

int find_max_cross(int i, int j) {
	int x_hop = 1;	
	while (is_valid(i + x_hop, j) && is_valid(i - x_hop, j)) x_hop++;
	x_hop--;
	int y_hop = 1;
	while (y_hop <= x_hop && is_valid(i, j + y_hop) && is_valid(i, j - y_hop)) y_hop++;
	y_hop--;
	int hop = y_hop; // hop = min(x_hop, y_hop);
	return hop;
}

int main() {
	ios::sync_with_stdio(false);
	int area = 0;
	int count_g = 0;
	cin >> n >> m;
	for(int i=0;i<n;i++) {
		for(int j=0;j<m;j++) {
			cin >> table[i][j];
			if (is_good(i, j)) count_g++;
		}
	}
	if (count_g >= 2) area = 1;
	for(int i=0;i<n;i++) {
		for(int j=0;j<m;j++) {
			if (!is_good(i,j)) continue;
			int cross_1 = find_max_cross(i, j);
			for(int step1 = 1; step1 <= cross_1; step1++) {
				set_cross(i, j, step1, 'B');
				for(int k=i;k<n;k++) {
					for(int l=0;l<m;l++) {
						if (!is_good(k, l)) continue;
						int cross_2 = find_max_cross(k, l);
						area = max(area, (step1*4+1)*(cross_2*4+1));
					}
				}
				set_cross(i, j, cross_1, 'G');
			}
		}
	}

	cout << area << endl;

	return 0;
}
