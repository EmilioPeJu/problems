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
    int x, y, z, n;
    int a, b, c;
    x=y=z=0;
    cin >> n;
    for(int i=0;i<n;i++) {
        cin >> a >> b >> c;
        x+=a; y+=b; z+=c;
    }
    if (x==0 && y==0 && z==0)
        cout << "YES" << endl;
    else
        cout << "NO" << endl;
    return 0;
}
