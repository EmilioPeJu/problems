#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
#define MAX_DIM 1001

char table_done[MAX_DIM][MAX_DIM];
char table[MAX_DIM][MAX_DIM];
int n, m;

bool filled_around(int i, int j) {
	if (i <= 0 || i >= (n-1) || j<=0 || j>=(m-1)) return false;
	for(int x=i-1;x<=i+1;x++) {
		for(int y=j-1;y<=j+1;y++) {
			if (!(x==i && y==j) && table[x][y]=='.') return false;
		}
	}
	return true;
}

void fill_done(int i, int j) {
	for(int x=i-1;x<=i+1;x++) 
		for(int y=j-1;y<=j+1;y++) 
			table_done[x][y] = '#';
}

int main() {
	ios::sync_with_stdio(false);
	cin >> n >> m;
	bool all_hash = true;
	for(int i=0;i<n;i++) {
		for(int j=0;j<m;j++) {
			cin >> table[i][j];
			table_done[i][j] = '.';
			if (table[i][j]!='#') all_hash = false;
		}
	}
	// corner case, not possible to use the pen
	if (all_hash && n<=3 && m<=3) {
		cout << "NO" << endl;
		return 0;
	}
	for(int i=0;i<n;i++) {
		for(int j=0;j<m;j++) {
			if (table[i][j] == '.' && filled_around(i, j)) {
				fill_done(i, j);
				table_done[i][j] = '.';
			}
		}
	}
	for(int i=0;i<n;i++) {
		for(int j=0;j<m;j++) {
			if (table[i][j] == '#' && filled_around(i, j)) {
				fill_done(i, j);
			}
		}
	}
	bool res = true;
	for(int i=0;i<n;i++) {
		for(int j=0;j<m;j++) {
			if(table[i][j]!=table_done[i][j]) {
				res=false;
				break;
			}
		}
	}
	cout << (res?"YES":"NO") << endl;
	// another way, mark forbiden places (adjacents to . and border)
	// then check that for every # there is a non forbidden neighbor
	return 0;
}
