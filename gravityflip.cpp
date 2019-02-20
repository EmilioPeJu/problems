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
    int arr[128];
    cin >> n;
    for(int i=0;i<n;i++) {
        cin >> arr[i];
    }
    sort(arr, arr+n);
    for(int i=0;i<n-1;i++) {
        cout << arr[i] << " ";
    }
    cout << arr[n-1] << endl;
    return 0;
}
