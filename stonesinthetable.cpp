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
    string input;
    int n;
    cin >> n;
    cin >> input;
    int left=0, right=1;
    int res=0;
    while (left<n) {
        while (right < n && input[left] == input[right]) {
            right++;
            res++;
        }
        left = right;
        right++;
    }
    cout << res << endl;
    return 0;
}
