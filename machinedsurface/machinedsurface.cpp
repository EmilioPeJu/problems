#include <bits/stdc++.h>
using namespace std;
const int MAX_LEN=25;

int main() {
	while (true) {
		int n;
		char line[MAX_LEN + 1];
		int min_blanks=MAX_LEN;
		int total_blanks=0;
		cin >> n;
		if (n==0) break;
		for (int i=0;i<n;i++) {
			char current;
			cin >> current;
			int cnt=0;
			while (current!='\n') {
				cin.get(current);
				if (current == 'B' || current == ' ') cnt++;
			} 
			min_blanks=min(min_blanks, cnt);
			total_blanks+=cnt;
		}
		if (cin.eof()) break;
		cout << total_blanks-min_blanks*n << endl;
	}
	return 0;
}
