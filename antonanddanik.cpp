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
    cin >> n;
    string input;
    cin >> input;
    int cnt1=0, cnt2=0;
    for(char c : input) {
        if (c=='A') cnt1++;
        else if (c=='D') cnt2++;
    }
    if (cnt1 > cnt2)
        cout << "Anton" << endl;
    else if (cnt1 < cnt2)
        cout << "Danik" << endl;
    else
        cout << "Friendship" << endl;
    return 0;
}
