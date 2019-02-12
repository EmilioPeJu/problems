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
    int n,k, w;
    cin >> k >> n >> w;
    int cost = (k+w*k)*w/2;
    int result = max(cost - n, 0);
    cout << result << endl;
    return 0;
}
