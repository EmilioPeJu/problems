#include <bits/stdc++.h>
using namespace std;

int main() {
	int n;
	string line;
	cin >> n;
	getline(cin, line); // skip last line feed
	for(int tc=1;tc<=n;tc++) {
		int cnt = 0;
		char car;
		getline(cin, line);
		for(size_t i=0;i<line.length();i++) {
			car = line[i];
			if(car <= 'o' && car >= 'a') {
				cnt += 1 + (car - 'a') % 3;
			} else if (car >= 'p' && car <='s') {
				cnt += 1+ (car - 'p');
			} else if (car >= 'w' && car <='z') {
				cnt += 1+ (car - 'w');
			} else if (car >= 't' && car <='v') {
				cnt += 1+ (car - 't');
			}
			else {
				cnt += 1;
			}
		}
		cout << "Case #" << tc << ": " << cnt << endl;
	}

	return 0;
}
