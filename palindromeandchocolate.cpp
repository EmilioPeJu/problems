#include <iostream>
#include <string>
#include <algorithm>
using namespace std;

bool is_pal(long long x)
{
    string s = to_string(x);
    for(int i=0;i<s.size()/2;i++) {
        if (s[i] != s[s.size() - i - 1])
            return false;
    }
    return true;
}

long long get_odd_pal(int x)
{
    long long index = 0;
    long long dec = 1;
    while(true) {
        for(int i=dec;i<dec*10;i++) {
            if (is_pal(i)) {
                index++;
                if (index == x) return i;
            }
        }
        dec*=100;
    }
}

string faster_get_odd_pal(int x)
{
    string s = to_string(x);
    string t = s.substr(0, s.size()-1);
    reverse(begin(t), end(t));
    return s + t;
}

int main(int argc, char **argv)
{
    int tc;
    cin >> tc;
    for(int t=1;t<=tc;t++) {
        int n;
        cin >> n;
        if (n < 10) cout << "Case " << t << ": " << n << endl;
        else cout << "Case " << t << ": " << faster_get_odd_pal(n) << endl;
    }
    return 0;
}
