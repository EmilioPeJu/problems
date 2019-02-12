#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int arr[55];

bool comp(int x, int y) {
    return x>y;
}

int main()
{
    ios::sync_with_stdio(false);
    int n, k;
    cin >> n >> k;
    for(int i=0;i<n;i++) {
        cin>> arr[i];
    }
    int res = upper_bound(arr, arr+n, arr[k-1], comp) - arr;
    int loosers = 0;
    for(int i=0;i<res;i++) {
        if (arr[i] == 0) loosers++;
    }
    cout << res - loosers << endl;
    return 0;
}
