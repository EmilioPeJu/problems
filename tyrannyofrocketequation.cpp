#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

void part1() {
  long long total = 0;
  while (1) {
    long long in;
    cin >> in;
    if (cin.eof()) break;
    total += max(in / 3 - 2, 0LL);
  }
  cout << total << endl;
}

void part2() {
  long long total = 0;
  while (1) {
    long long in;
    cin >> in;
    while (in) {
      in = max(in / 3 - 2, 0LL);
      total += in;
    }
    if (cin.eof()) break;
    total += max(in / 3 - 2, 0LL);
  }
  cout << total << endl;
}

int main()
{
  ios::sync_with_stdio(false);
  // part1();
  part2();
  return 0;
}
