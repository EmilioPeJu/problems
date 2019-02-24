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
    int current; 
    int cnt[2] = {0};
    cin >> n;
    for(int i=0;i<n;i++) {
        cin >> current;
        if (current > 0)
            cnt[0]++;
        else if(current < 0)
            cnt[1]++;
    }
    if (cnt[0] >= (n+1)/2)
        cout << "1" << endl;
    else if (cnt[1] >= (n+1)/2)
        cout << "-1" << endl;
    else
        cout << "0" << endl;
    return 0;
}
