#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int arr[101];
int main()
{
    ios::sync_with_stdio(false);
    int n;
    int sum=0;
    int part=0;
    cin >> n;
    for(int i=0;i<n;i++) {
        cin >> arr[i];
        sum += arr[i];
    }
    sort(arr, arr+n);
    int i=n-1;
    while (part <= sum/2)
        part += arr[i--];
    cout << n - i - 1 << endl;
    return 0;
}
