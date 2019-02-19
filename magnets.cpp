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
    int n;
    cin >> n;
    char last;
    char current;
    string input;
    cin >> input;
    last = input[1];
    int res = 1;
    for(int i=1;i<n;i++) {
        cin >> input;
        current = input[1];
        if (last != current) res++;
        last = current;
    }
    cout << res << endl;
    return 0;
}
