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
    int n, m, a;
    cin >> n >> m >> a;
    int n_a = n/a;
    if (n%a) n_a++;
    int m_a = m/a;
    if (m%a) m_a++;
    unsigned long long res = (unsigned long long) n_a*m_a;
    cout << res << endl;
    return 0;
}
