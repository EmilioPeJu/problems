#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main()
{
    ios::sync_with_stdio(false);
    string input;
    cin >> input;
    int first = 0;
    int second = 0;
    for(auto c : input) {
        if (c=='0') {
            first++;
            second = 0;
        } else {
            second++;
            first = 0;
        }
        if (first >= 7 || second >= 7) {
            cout << "YES" << endl;
            return 0;
        }
    }
    cout << "NO" << endl;

    return 0;
}
