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
    int one_i, one_j, num;
    for(int i=0;i<5;i++) {
        for(int j=0;j<5;j++) {
            cin >> num;
            if (num == 1) {
                one_i = i;
                one_j = j;
            }
        }
    }
    int result = abs(2-one_i) + abs(2-one_j);
    cout << result << endl;
    return 0;
}
