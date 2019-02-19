#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
bool done[128];
int main()
{
    ios::sync_with_stdio(false);
    int n, p;
    int current;
    cin >> n;
    cin >> p;
    for(int i=0;i<p;i++) {
        cin >> current;
        done[current-1] = true;
    }
    cin >> p;
    for(int i=0;i<p;i++) {
        cin >> current;
        done[current-1] = true;
    }
    for(int i=0;i<n;i++) {
        if (!done[i]) {
            cout << "Oh, my keyboard!" << endl;
            return 0;
        }
    }
    cout << "I become the guy." << endl;
    return 0;
}
