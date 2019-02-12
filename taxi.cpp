#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int cnt[5];
int main()
{
    ios::sync_with_stdio(false);
    int n, x;
    cin >> n;
    for(int i=0;i<n;i++) {
        cin >> x;
        cnt[x]++;
    }
    unsigned long long res = 0;

    // one and three
    int thirdAndOne= min(cnt[1], cnt[3]);
    res += thirdAndOne;
    cnt[1] -= thirdAndOne;
    cnt[3] -= thirdAndOne;

    // one*2 and two
    int twoAndOne = min(cnt[1] / 2, cnt[2]);
    res += twoAndOne;
    cnt[1] -= 2*twoAndOne;
    cnt[2] -= twoAndOne;

    // two and two
    res += cnt[2] / 2;
    cnt[2] %= 2;

    // one*4
    res += cnt[1] / 4;
    cnt[1] %= 4;

    // one*1 and two
    twoAndOne = min(cnt[1], cnt[2]);
    res += twoAndOne;
    cnt[2] -= twoAndOne;
    cnt[1] -= twoAndOne;

    // any one left will be in 1 group
    if (cnt[1]) cnt[1] = 1;

    res += cnt[1] + cnt[2] + cnt[3] + cnt[4];
    cout << res << endl;
    return 0;
}
