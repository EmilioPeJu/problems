#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main()
{
    string vowels = "aeiouAEIOUyY";
    ios::sync_with_stdio(false);
    string input;
    cin >> input;
    for (auto c : input) {
        if (vowels.find(c)!=-1) continue;
        cout << "." << (char) tolower(c);
    }
    cout << endl;
    return 0;
}
