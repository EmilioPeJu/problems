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
    string a;
    string b;
    cin >> a;
    cin >> b;
    int result = 0;
    for(int i=0;i<a.size();i++) {
        char c_a = tolower(a[i]);
        char c_b = tolower(b[i]);
        if (c_a < c_b) {
            result = -1;
            break;
        } else if (c_a > c_b) {
            result = 1;
            break;
        }
    }
    cout << result << endl;
    return 0;
}
