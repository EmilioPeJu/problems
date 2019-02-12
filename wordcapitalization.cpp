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
    cin >> input;
    if (input[0] <= 'z' && input[0] >= 'a')
        input[0] = input[0] + 'A' - 'a';
    cout << input << endl;
    return 0;
}
