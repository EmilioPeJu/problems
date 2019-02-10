#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

long long dp[1000][18279]; 
string input[1000][18279];
bool processed[1000][18279];

vector<string> split_formula(string formula) {
	vector<string> res;
	string buffer = "";
	for (char car: formula) {
		if (car != '+')
			buffer += car;
		else {
			res.push_back(buffer);
			buffer = "";
		}
	}
	if (buffer != "")
		res.push_back(buffer);
	return res;
}

pair<size_t, size_t> parse_cell_index(string cell) {
	size_t col=0, row=0;
	size_t car_i=0;
	for (char car: cell) {
		if(car <= 'Z' && car >= 'A') {
			col = col*('Z' - 'A' + 1) + (car - 'A');
			car_i++;
		}
		else if (car >= '0' && car <= '9')
			row = row*10 + (car - '0');
	}
	if (car_i > 1) col += 26;
	if (car_i > 2) col += 26*26;
	return make_pair(row - 1, col);
}

// recursive solution is OK, as the tests never 
// overflow the stack
long long eval_cell(size_t row, size_t col) {
	if (processed[row][col])
		return dp[row][col];
	if (input[row][col].size() == 0)
		return 0;
	if (input[row][col][0] != '=') 
	{
		processed[row][col] = true;
		return dp[row][col] = stoll(input[row][col]);
	}
	long long res = 0;
	for (string token: split_formula(input[row][col])) {
		pair<size_t, size_t> row_col_token = parse_cell_index(token);
		res += eval_cell(row_col_token.first, row_col_token.second);
	}
	processed[row][col] = true;
	return dp[row][col] = res;
}

int main() {
	ios::sync_with_stdio(false);
	size_t n_col, n_row;
	int tc;
	cin >> tc;
	while(tc--) {
		cin >> n_col >> n_row;
		for(size_t i=0;i<n_row;i++) {
			for(size_t j=0;j<n_col;j++) {
				cin >> input[i][j];
				processed[i][j] = false;
			}
		}
		for(size_t i=0;i<n_row;i++) {
			for(size_t j=0;j<n_col;j++) {
				cout << eval_cell(i, j);
				if (j != n_col - 1) cout << " ";
			}
			cout << endl;
		}
	}
	return 0;
}
