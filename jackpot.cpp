#include <bits/stdc++.h>
#define MAX_DIM 1001
using namespace std;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	while(true) {
		int n, num;
		cin >> n;
		if (n == 0) break;
		long long res = 0, acc=0;
		for(int i=0;i<n;i++) {
			cin >> num;
			acc += num;
			res = max(res, acc);
			if(acc<0) acc=0;
		}
		if (res == 0)
			cout << "Losing streak." << endl;
	        	
		else
			cout << "The maximum winning streak is " << res << "." << endl;
	}

	return 0;
}
