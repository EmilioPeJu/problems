#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int arr[300001];
int main()
{
    ios::sync_with_stdio(false);
    int n;
    cin >> n;

    for(int i=0;i<n;i++)
        cin >> arr[i];
    long long res=0;
    int left=0, right=n-1;
    sort(arr, arr+n);
    while (left<right) {
        long  sum = arr[left]+arr[right];
        res += sum*sum;
        left++; right--;
    }
    
    cout << res << endl;

    return 0;
}
