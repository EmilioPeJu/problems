#include <bits/stdc++.h>
#include <string.h>
#define MAXN 1000000
using namespace std;
long table[MAXN + 1];

long iter_func(long n) {
		long res = 1;
		while(n>1) {
				if (n%2 == 1)
						n = n*3 + 1;
				else
						n = n/2;
				res++;
		}
		return res;
}

long func(int n) {
		if (table[n] == 0)
			table[n] = iter_func(n);
		return table[n];
}

int main() {
	memset(table, 0, (MAXN + 1)*sizeof(long));
	table[1] = 1;
	while(true) {
			long n, i, j;
			cin >> i >> j;
			n=0;
			if (cin.eof()) break;
			for(long k=min(i,j);k<max(i, j)+1;k++) {
				n=max(n, func(k));	
			}
			cout << i << " " << j << " " << n << endl;
	}
	return 0;
}
