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
    if(input1.size() != input2.size())
        goto fail;
    for(int i=0;i<input1.size();i++) {
        if (input1[i] != input2[input1.size() -i -1])
            goto fail;
    }
    cout << "YES" << endl;
    return 0;
fail:
    cout << "NO" << endl;
    return 0;
}
