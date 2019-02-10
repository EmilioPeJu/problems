#include <bits/stdc++.h>
using namespace std;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	int T;
	int arr[3];
	cin >> T;
	for(int tc=1;tc<=T;tc++) {
		for(int i=0;i<3;i++) {
			cin >> arr[i];
		}
		sort(arr, arr + 3);
		cout << "Case "<< tc << ": " << arr[1] << endl;
	}

	return 0;
}
