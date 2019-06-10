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
  int n;
  int arr[3001];
  while (1) {
    cin >> n;
    if (n == 0) continue;
    if (cin.eof()) break;
    for (int i=0;i<n;i++)
      cin >> arr[i];
    for(int i=0;i<(n-1);i++) {
      arr[i] = abs(arr[i+1] - arr[i]);
    }
    sort(arr, arr+n-1);
    bool isjolly = true;
    for(int i=0;i<(n-1);i++) {
      if (arr[i] != i + 1) {
        isjolly = false;
        break;
      }
    }
    if (isjolly)
      cout << "Jolly\n";
    else
      cout << "Not jolly\n";
  }
  return 0;
}
