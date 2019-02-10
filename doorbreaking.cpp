#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main()
{
    int n, x, y;
    unsigned long long breakingnow=0;
    int current;
    int res=0;
    cin >> n >> x >> y;
    for(int i=0;i<n;i++) {
        cin >> current;
        if (current <= x) breakingnow++;
    }
    if (x > y) res = n;
    else res = breakingnow/2 + breakingnow%2;
    cout << res << endl;
    ios::sync_with_stdio(false);
    return 0;
}
