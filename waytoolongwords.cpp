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
    for(int i=0;i<n;i++) {
        cin >> input;
        if (input.size() > 10)
            cout << input[0] << input.size() - 2 << input[input.size()-1] << endl;
        else
            cout << input << endl;
    }
    return 0;
}
