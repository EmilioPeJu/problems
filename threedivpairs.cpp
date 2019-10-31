#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
size_t cnt[3];

int main()
{
  ios::sync_with_stdio(false);
  size_t n;
  cin >> n;
  for(size_t i=0; i < n; i++) {
    size_t current;
    cin >> current;
    cnt[current % 3]++;
  }
  size_t result = 0;
  if (cnt[0]) {
    result += cnt[0]*(cnt[0] - 1) / 2;
  }
  result += cnt[1]*cnt[2];

  cout << result << endl;

  return 0;
}
