#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

bool is_lucky(int x) {
    while(x) {
        if (x%10 != 4 && x%10 != 7) return false;
        x /= 10;
    }
    return true;
}
int main()
{
    ios::sync_with_stdio(false);
    int n;
    cin >> n;
    if (is_lucky(n)) {
        cout << "YES" << endl;
        return 0;
    }
    for(int i=4;i<=(n/2);i++) {
        if (n%i==0 && is_lucky(i)) {
            cout << "YES" << endl;
            return 0;
        }
    }
    cout << "NO" << endl;
    return 0;
}
