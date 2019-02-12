#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

bool is_lucky(int x) {
    if (x == 0) return false;
    while (x) {
        int dig = x%10;
        if (dig!= 4 && dig!=7)
            return false;
        x/=10;
    }
    return true;
}

int main()
{
    ios::sync_with_stdio(false);
    int cnt=0;
    string input;
    cin >> input;
    for (char c : input)
        if (c=='4' || c=='7')
            cnt++;
    if(is_lucky(cnt))
        cout << "YES" << endl;
    else
        cout << "NO" << endl;

    return 0;
}
