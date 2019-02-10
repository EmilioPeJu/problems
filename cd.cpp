#include <bits/stdc++.h>
using namespace std;
const int MAX_N = 21;
const int MAX_TAPE = 1000;

int songs[MAX_N];
int n, tape;
typedef bitset<MAX_N> objects;
int max_value;
objects max_taken;

void _bt(objects& taken, int index, int value) {
	if (value > tape) return;
	if (index == n && value > max_value) {
		max_value = value;
		max_taken = taken;
	}
	if (index + 1 <= n) {
		taken.set(index);
		_bt(taken, index + 1, value + songs[index]);
		taken.reset(index);
		_bt(taken, index + 1, value);
	}
}

void bt() {
	objects taken;
	max_value = 0;
	taken.reset();
	_bt(taken, 0, 0);
}

int main() {
	while (true) {
		cin >> tape;
		if (cin.eof()) break;
		cin >> n;
		for(int i=0;i<n;i++) {
			cin >> songs[i];	
		}
		bt();
		for(int i=0;i<n;i++) {
			if(max_taken[i]) {
				cout << songs[i] << " ";
			}
		}
		cout << "sum:" << max_value << endl;
	}
	return 0;
}
