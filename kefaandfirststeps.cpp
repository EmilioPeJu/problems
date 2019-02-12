#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int arr[100001];
int m[100001];
int main()
{
    ios::sync_with_stdio(false);
    int n;
    cin >> n;
    for(int i=0;i<n;i++) {
        cin >> arr[i];
    }
    int result=1;
    int count=1;
    for(int i=1;i<n;i++) {
        if(arr[i] >= arr[i-1]) count++;
        else                   count=1;
        result = max(result, count);
    }
    cout << result << endl;
    return 0;
}
