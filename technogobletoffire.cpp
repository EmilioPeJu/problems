#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int main()
{
    int power[101];
    int school[101];
    int maxschool[101];
    memset(maxschool, 0, sizeof(int)*101);
    int n, m, k;
    int result = 0;
    ios::sync_with_stdio(false);
    cin >> n >> m >> k;
    for(int i=0;i<n;i++)
        cin >> power[i];
    for(int i=0;i<n;i++) {
        cin >> school[i];
    }
    for(int i=0;i<n;i++) {
        maxschool[school[i]] = max(maxschool[school[i]], power[i]);
    }
    for(int i=0;i<k;i++) {
        int chosen;
        cin >> chosen;
        if (power[chosen-1] < maxschool[school[chosen-1]]) result++;
    }
    cout << result << endl;
    return 0;
}
