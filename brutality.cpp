#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int arr[200001];
int order[200001];

int main()
{
    ios::sync_with_stdio(false);
    int n, k;
    string input;
    cin >> n >> k;
    for(int i=0;i<n;i++)
        cin >> arr[i];
    cin >> input;
    int left=0, right=0;
    long long res = 0;
    while (left < n) {
        while (right < n && input[left] == input[right])
            right++;
        if (right - left > k) {
            sort(arr+left, arr+right);
            left += right-left-k;
        }
        for(int i=left;i<right;i++) {
            res += arr[i];
        }
        left = right;
    }
    cout << res << endl;
    return 0;
}
