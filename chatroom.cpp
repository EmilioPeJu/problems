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
    string test = "hello";
    int i=0;
    cin >> input;
    for(char c : input) {
        if (test[i] == c) {
            i++;
            if (i>= test.size()) break;
        }
    }
    if (i>=test.size()) cout << "YES" << endl;
    else                cout << "NO" << endl;
    return 0;
}
