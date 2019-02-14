#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main()
{
    int arr[101];
    int res[101];
    ios::sync_with_stdio(false);
    int n;
    cin >> n;
    for(int i=0;i<n;i++) {
        cin >> arr[i];
        res[arr[i]-1] = i+1;
    }
    for(int i=0;i<n-1;i++) {
        cout << res[i] << " ";
    }
    cout << res[n-1] << endl;

    return 0;
}
