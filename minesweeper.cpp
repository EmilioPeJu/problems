#include <bits/stdc++.h>
#define MAX_DIM  101
using namespace std;

int main() {
	int n, m;
	char arr[MAX_DIM][MAX_DIM];
	int dirs[][2] = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}, {1, 1}, {-1, -1}, {1, -1}, {-1, 1}};
	int tc=1;
	while (true) {
		cin >> n >> m;
		if(n==0 && m==0) break;
		for(int i=0;i<n;i++) {
			cin >> arr[i];
		}
		for(int i=0;i<n;i++) {
			for(int j=0;j<m;j++) {
				if (arr[i][j] == '.') arr[i][j] = '0';
				if (arr[i][j] != '*') continue;
				for(int x=0;x<8;x++) {	
					int newi = i + dirs[x][0];
					int newj = j + dirs[x][1];
					if (newi >= 0 && newi < n && newj >= 0 && newj < m) {
						if (arr[newi][newj] == '*') continue;
						if (arr[newi][newj] == '.') arr[newi][newj] = '0';
						arr[newi][newj] += 1;
					}
				}
				
			}
		}
		if (tc != 1) { cout << endl; }
		cout << "Field #" << tc++ <<  ":" << endl;
		for(int i=0;i<n;i++) {
			cout << arr[i] << endl;
		}


	}

	return 0;
}
