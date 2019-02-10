#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

class BITree {
#define ls1(x) (x&-x)
	private:
		vector<int> freq;
		vector<int> sum;
	public:
		BITree(int n) {
			freq.assign(n, 0);
			sum.assign(n+1, 0);
		}

		void adjust(int index, int val) {
			freq[index] += val;
			// index used in freq and sum differs 1
			// because of the dummy root/first element
			index++;
			while (index < (int) sum.size()) {
				sum[index]+=val;
				index+=ls1(index);
			}
		}

		void set(int index, int val) {
			adjust(index, val - freq[index]);
		}

		int query(int index) {
			// query interval [0, index]
			int result=0;
			index++;
			while (index > 0) {
				result+=sum[index];
				index-=ls1(index);
			}
			return result;
		}

		int query(int a, int b) {
			return query(b) - (b==0?0:query(a-1));
		}
};

int main() {
	ios::sync_with_stdio(false);
	int tc=1;
	bool isfirst=true;
	while (true) {
		int n;
		cin >> n;
		if (n == 0) break;
		BITree tree(n);
		int current;
		for(int i=0;i<n;i++) {
			cin >> current;
			tree.adjust(i, current);
		}
		if (!isfirst) cout << endl;
		isfirst = false;
		cout << "Case " << tc++ << ":" << endl;
		string  command;
		int x, y;
		while (true) {
			cin >> command;
			if (command == "END") break;
			cin >> x >> y;
			if (command == "S") {
				tree.set(x-1, y); // y is value
			} else if (command == "M") {
				cout << tree.query(x-1, y-1) << endl;
			}
		}
	}
	return 0;
}
