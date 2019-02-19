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
    long long n;
    cin >> n;
    if (n==1) {
        cout << -1 << endl;
        return 0;
    }
    long long result;
    if (n%2==0) {
        result = -1 + n - (n-2) / 2;
    } else {
        result = -1 - (n-1) / 2;
    }
    cout << result << endl;
    return 0;
}
