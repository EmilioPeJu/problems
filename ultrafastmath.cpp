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
    string input1, input2;
    cin >> input1;
    cin >> input2;
    for(int i=0;i<(int)min(input1.size(), input2.size());i++)
        cout <<  static_cast<int>((input1[i]-'0')^(input2[i]-'0'));
    cout << endl;
    return 0;
}
