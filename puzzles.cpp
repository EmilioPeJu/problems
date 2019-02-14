#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int arr[64];
int main()
{
    ios::sync_with_stdio(false);
    int n, m;
    cin >> n >> m;
    for(int i=0;i<m;i++) {
        cin >> arr[i];
    }
    sort(arr, arr+m);
    int res = INT_MAX;
    for(int i=0;i<=m-n;i++) {
        res = min(res, arr[i+n-1]-arr[i]);
    }
    cout << res << endl;
    return 0;
}
