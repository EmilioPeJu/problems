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
    int x, y, z;
    int a, b, c;
    cin >> x >> y >> z;
    cin >> a >> b >> c;
    if (a<x) goto fail;
    a -= x;
    if (y > (a+b) || z > (a+b+c-y)) goto fail;
    cout << "YES" << endl;
    return 0;
fail:
    cout << "NO" << endl;
    return 0;
}
