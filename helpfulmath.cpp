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
    sort(input.begin(), input.end());
    for(int i=input.size()/2;i<input.size()-1;i++) {
        cout << input[i] << "+";
    }
    cout << input[input.size()-1] << endl;


    return 0;
}
