#include <bits/stdc++.h>
#define MAXN 256
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int main()
{
  ios::sync_with_stdio(false);
  size_t n;
  unsigned int initial = 0;
  unsigned int result;
  vector<pair<unsigned int, unsigned int> > pars;
  bitset<MAXN> state;
  state.reset();
  string inseq;
  cin >> n;
  cin >> inseq;
  for(size_t i=0; i < n; i++) {
    if (inseq[i] == '1') {
      initial++;
      state.set(i);
    }
  }
  result = initial;
  if (result == n) goto show_sol;
  for(size_t i=0; i < n; i++) {
    unsigned int a, b;
    cin >> a >> b;
    pars.push_back(make_pair(a, b));
  }
  for(size_t x=1; x < MAXN; x++) {
    for(size_t pi=0; pi < pars.size(); pi++) {
      unsigned int a = pars[pi].first;
      unsigned int b = pars[pi].second;
      if (x >= b && (x-b)%a == 0)
        state.flip(pi);
    }
    result = max(result, (unsigned int) state.count());
  }
show_sol:
  cout << result << endl;
  return 0;
}
