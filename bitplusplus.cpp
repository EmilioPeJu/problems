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
    int x = 0;
    string input;
    for(int i=0;i<n;i++) {
        cin >> input;
        if (input.find("++") != string::npos) x++;
        else if (input.find("--") != string::npos) x--;
    }
    cout << x << endl;
    return 0;
}
