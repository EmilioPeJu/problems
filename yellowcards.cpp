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
  unsigned int a1, a2, k1, k2, n;
  cin >> a1 >> a2 >> k1 >> k2 >> n;
  unsigned int rem1 = min(n/k1, a1);
  unsigned int rem2 = min(n/k2, a2);
  unsigned int case1 =  rem1 + min((n - rem1*k1)/k2, a2);
  unsigned int case2 =  rem2 + min((n - rem2*k2)/k1, a1);
  unsigned int maxcards = max(case1, case2);
  unsigned int mincards = 0;
  unsigned int givethemall = (k1 - 1) * a1 + (k2 - 1) * a2;
  if (n > givethemall) mincards += (n - givethemall);
  cout << mincards << " " << maxcards << endl;
  return 0;
}
