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
    int x;
    cin >> x;
    int res = x/5;
    if (x%5) res++;
    cout << res << endl;
    return 0;
}
