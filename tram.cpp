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
    int a, b;
    unsigned long long res = 0;
    unsigned long long current = 0;
    cin >> n;
    for(int i=0;i<n;i++) {
        cin >> a >> b;
        current += (b-a);
        res = max(res, current);
    }
    cout << res << endl;
    return 0;
}
