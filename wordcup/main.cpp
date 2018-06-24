#include <bits/stdc++.h>
#define MAX_N 100000
using namespace std;

int one_round(int *queues, int n, int pos) {
	int cnt=0;
	for (int i=pos; i<(pos + n); i++) {
		if (queues[i%n] - cnt <= 0)
			return i%n;
		cnt++;
	}
	return -1;
}
	

int main() {
	int n;
	int queues[MAX_N];
	cin >> n;
	for(int i=0; i<n; i++) {
		cin >> queues[i];
	}
	int pos = 0;
	while (true) {
		int val = one_round(queues, n, pos);
		if (val == -1) {
			int* min_ite=min_element(queues, queues + n);
			int min_val = *min_ite;
			pos = (pos + min_val) % n;
			for(int i=0;i<n;i++) queues[i]-=min_val;
		} else {
			cout << val + 1 << endl;
			break;
		}
	}
	return 0;
}
