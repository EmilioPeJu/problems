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
    int m, n;
    cin >> m >> n;
    if (m==1 && n==1) {
        cout << 0 << endl;
        return 0;
    }
    int res = m*n/2;
    cout << res << endl;
    return 0;
}
