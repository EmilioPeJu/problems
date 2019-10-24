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
  int tc;
  cin >> tc;
  while(tc--) {
    long long p_odd = 0;
    long long p_even = 0;
    long long q_odd = 0;
    long long  q_even = 0;
    size_t n, m;
    long long p, q;
    cin >> n;
    for(size_t i=0; i < n; i++) {
      cin >> p;
      if (p % 2 == 0) p_even++;
      else            p_odd++;
    }
    cin >> m;
    for(size_t i=0; i < m; i++) {
      cin >> q;
      if (q % 2 == 0) q_even++;
      else            q_odd++;
    }
    cout << p_even*q_even + p_odd*q_odd << endl;
  }
  return 0;
}
