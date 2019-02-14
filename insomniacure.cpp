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
    int parameters[4];
    bool there_is_one=false;
    for(int i=0;i<4;i++) {
        cin >> parameters[i];
    }
    int d;
    cin >> d;
    int result = 0;
    for(int i=1;i<=d;i++) {
        for(int j=0;j<4;j++) {
            if (i%parameters[j]==0) {
                result++;
                break;
            }
        }
    }
    cout << result << endl;
    return 0;
}
