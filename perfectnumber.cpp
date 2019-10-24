#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

bool is_good(unsigned long long x) {
  unsigned long long s=0;
  while (x && s<=10) {
   s += (x % 10);
   x /= 10;
  }
  return s == 10;
}

int main()
{
  ios::sync_with_stdio(false);
  size_t k;
  cin >> k;
  size_t index = 0;
  unsigned long long current=18;
  while (index < k) {
    current += 1;
    if (is_good(current))
      index++;
  }
  cout << current << endl;
  return 0;
}
