#include <bits/stdc++.h>
using namespace std;
#define INF 4000000000
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

struct Node {
  size_t count;
  char last;
  size_t index;
};

struct Node adapt(string in) {
  size_t count = 0;
  char last;
  for (char c: in) {
    if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
      last = c;
      count++;
    }
  }
  return  (struct Node) { .count=count, .last=last };
}

bool comp(const struct Node &p1, const struct Node &p2)
{
  size_t s1 = p1.count;
  size_t s2 = p2.count;
  if (s1 == s2)
    return p1.last < p2.last;
  return s1 < s2;
}

int main()
{
  ios::sync_with_stdio(false);
  string input;
  size_t n;
  cin >> n;
  vector<string> words;
  vector<struct Node> adapted;
  vector<pair<size_t, size_t> > result1;
  vector<pair<size_t, size_t> > result2;

  size_t index=0;
  while (n--) {
    cin >> input;
    words.push_back(input);
    struct Node new_node = adapt(input);
    new_node.index = index++;
    adapted.push_back(new_node);
  }
  sort(adapted.begin(), adapted.end(), comp);
  bitset<100001> used;
  used.reset();
  for (size_t i=0; i < (adapted.size() - 1); i++) {
    size_t j = i + 1;
    if (adapted[i].count == adapted[j].count && adapted[i].last == adapted[j].last) {
      result1.push_back(make_pair(adapted[i].index, adapted[j].index));
      used.set(i);
      used.set(j);
      i++;
    }
  }
  size_t last_count = INF;
  size_t last_index = INF;
  for (size_t i=0; i < adapted.size(); i++) {
    if (used[i]) continue;
    if (last_count == adapted[i].count) {
      result2.push_back(make_pair(adapted[i].index, last_index));
      last_index = INF;
      last_count = INF;

    } else {
      last_index = adapted[i].index;
      last_count = adapted[i].count;
    }
  }
  while (result1.size() > result2.size()) {
    result2.push_back(result1.back());
    result1.pop_back();
  }
  size_t result = min(result1.size(), result2.size());
  cout << result << endl;
  for (size_t i=0; i < result; i++) {
    size_t i1 = result2[i].first;
    size_t i2 = result1[i].first;
    size_t i3 = result2[i].second;
    size_t i4 = result1[i].second;
    cout << words[i1] << " " << words[i2] << endl;
    cout << words[i3] << " " << words[i4] << endl;
  }
  return 0;
}
