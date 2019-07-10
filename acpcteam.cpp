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
  int n, m;
  bitset<512> people[512];
  cin >> n >> m;
  string input;
  for(int i=0; i < n; i++) {
    cin >> input;
    for(int j=0; j < m; j++) {
      if (input[j] == '1')
        people[i].set(j);
    }
  }
  size_t maxtopics = 0;
  size_t maxnum = 0;
  for(int i=0; i < n; i++) {
    for(int j=i+1; j < n; j++) {
      bitset<512> current = people[i] | people[j];
      if (current.count() == maxtopics)
        maxnum++;
      else if (current.count() > maxtopics) {
        maxnum = 1;
        maxtopics = current.count();
      }
    }
  }
  cout << maxtopics << endl << maxnum << endl;
  return 0;
}
