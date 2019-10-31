#include <bits/stdc++.h>
//#include <regex>
#include <cstdio>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

static void transform1(int row, int col)
{
  string coll;
  while (col) {
    // - 1 considers variations of each smaller size
    // except first which considers passing to 0-based
    col--;
    coll +=  'A' + (col % 26) ;
    col /= 26;
  }
  // one way of avoiding to reverse is making this
  // function recursive, and printing after the recursive call
  reverse(coll.begin(), coll.end());
  cout << coll << row << endl;
}

static void transform2(int row, char *coll) {
  int x = 0;
  for(size_t i=0; i < strlen(coll); i++) {
    // + 1 considers variations of each smaller size
    // except first which considers passing to 1-based
    x = x*26 + (coll[i] - 'A' + 1);
  }
  cout << "R" <<  row << "C" << x << endl;
}

// too slow
/* static bool is_format1(string input) { */
/*   regex reg("R[0-9]+C[0-9]+"); */
/*   return regex_match(input, reg); */
/* } */

char input[256];
int main()
{
  ios::sync_with_stdio(false);
  int tc;
  scanf("%d", &tc);
  while(tc--) {
    int row;
    int col;
    scanf("%s", input);
    if (sscanf(input, "R%dC%d", &row, &col) == 2) {
      // like getting to 26 base but substracting 2^1 + 2^2 .. 2^(length-1)
      transform1(row, col);
    } else {
      char col_letters[64];
      size_t index = 0;
      char *p = input;
      int row = 0;
      for (p=input; *p >= 'A'; p++)
        col_letters[index++] = *p;
      col_letters[index] = '\0';
      sscanf(p, "%d", &row);
      // like getting from 26 base but adding 2^1 + 2^2 .. 2^(length-1)
      transform2(row, col_letters);
    }
  }
  return 0;
}
