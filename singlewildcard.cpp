#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main() {
	ios::sync_with_stdio(false);
	int  n, m;
	bool result;
	cin >> n >> m;
	string pattern;
	string input;
	cin >> pattern;
	cin >> input;
	if (pattern.find('*') == -1)  {
		result = pattern == input;
	} else {
		int in_i=0, in_j=input.size() - 1;
		int pa_i=0, pa_j=pattern.size() - 1;
		while (in_i < input.size() && pa_i < pattern.size() && pattern[pa_i]!='*' && pattern[pa_i] == input[in_i]) { 
			in_i++; pa_i++;
		}
		while (in_j >= 0 && pa_j >= 0 && pattern[pa_j] !='*' && pattern[pa_j]==input[in_j]) {
			in_j--; pa_j--;
		}


		result = (pa_i < pattern.size() && pa_j >=0 && pa_i==pa_j) && (in_i-1) <= in_j;
	}
	cout << (result ? "YES" : "NO") << endl;


	return 0;
}
