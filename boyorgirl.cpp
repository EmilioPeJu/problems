#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

bool  carset['z'-'a'+1];
int main()
{
    ios::sync_with_stdio(false);
    string input;
    cin >> input;
    for (char c : input)
        carset[c-'a'] = true;
    int res=0;
    for(int i=0;i<'z'-'a'+1;i++) {
        res += (int) carset[i];
    }
    if (res % 2)
        cout << "IGNORE HIM!" << endl;
    else
        cout << "CHAT WITH HER!" << endl;

    return 0;
}
