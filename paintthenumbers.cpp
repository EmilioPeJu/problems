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
  unsigned int arr[101];
  bitset<101> taken;
  taken.reset();
  size_t n;
  cin >> n;
  for(size_t i=0; i < n; i++) {
    cin >> arr[i];
  }
  sort(arr, arr+n);
  unsigned int colors = 0;
  unsigned int index = 0;
  while (taken.count() < n) {
    if (taken[index]) {
      index++;
      continue;
    }
    taken.set(index);
    colors++;
    for(size_t i=index+1; i < n; i++) {
      if (arr[i] % arr[index] == 0) {
        taken.set(i);
      }
    }
    index++;
  }
  cout << colors << endl;
  return 0;
}
