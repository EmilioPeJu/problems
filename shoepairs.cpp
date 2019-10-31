#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
size_t shoes[2][101];

int main()
{
  ios::sync_with_stdio(false);
  size_t n;
  cin >> n;
  size_t result = 0;
  for(size_t i=0; i < n; i++) {
    int num;
    char side;
    cin >> num >> side;
    shoes[side == 'L'?0:1][num]++;
  }
  for(size_t i=1; i < 101; i++) {
    result += min(shoes[0][i], shoes[1][i]);
  }
  cout << result << endl;
  return 0;
}
