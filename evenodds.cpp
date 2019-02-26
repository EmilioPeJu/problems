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
    long long n, k;
    cin >> n >> k;
    if (k <= (n+1)/2)
        cout << (k-1)*2+1 << endl;
    else
        cout  << (k-(n+1)/2)*2 << endl;
    return 0;
}
