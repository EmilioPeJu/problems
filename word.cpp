#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
string input;
int main()
{
    ios::sync_with_stdio(false);
    cin >> input;
    int cnt=0;
    for(char c : input)
        if (c>='a' && c<='z')
            cnt++;

    if (cnt >= (input.size()+1)/2)
        transform(input.begin(), input.end(), input.begin(), ::tolower);
    else
        transform(input.begin(), input.end(), input.begin(), ::toupper);

    cout << input << endl;
    return 0;
}
