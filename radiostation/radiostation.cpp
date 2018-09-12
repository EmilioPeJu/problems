#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int n, m;
	map <string, string> ip_name;
	cin >> n >> m;
	string name, ip, command1, command2;
	for(int i=0;i<n;i++) {
		cin >> name >> ip;
		ip_name[ip + ";"] = name;
	}
	for(int i=0;i<m;i++) {
		cin >> command1 >> command2;
		cout << command1 << " " << command2 << " #" + ip_name[command2] << endl;
	}

	return 0;
}
