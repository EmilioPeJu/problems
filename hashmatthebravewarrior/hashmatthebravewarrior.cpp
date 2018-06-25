#include <bits/stdc++.h>
using namespace std;

int main() {
	long n, m;

	while (true) {
		cin >> n;
		if (cin.eof()) break;
		cin >> m;
		long diff = m - n;
		if (diff < 0) diff*=-1;
		cout << diff << endl;
	}

	return 0;
}
