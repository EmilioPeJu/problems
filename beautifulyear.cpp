#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

bool is_beautiful(int year) {
    bool digits[10] = {false};
    while (year) {
        int d = year % 10;
        if (digits[d]) return false;
        digits[d] = true;
        year/=10;
    }
    return true;
}

int main()
{
    ios::sync_with_stdio(false);
    int year;
    cin >> year;
    year++;
    while(not is_beautiful(year)) year++;
    cout << year << endl;
    return 0;
}
