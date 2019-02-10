#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

inline bool isVowel(char x) {
    return x == 'a' || x == 'e' || x == 'i' || x == 'o' || x == 'u';
}

int main() {
	ios::sync_with_stdio(false);
        string s, t;
        cin >> s;
        cin >> t;
        if (s.size() != t.size()) {
            cout << "No" << endl;
            return 0;
        }

        for (int i=0; i < s.size(); i++) {
            if (isVowel(s[i])^isVowel(t[i])) {
                cout << "No" << endl;
                return 0;
            }
        }
        cout << "Yes" << endl;
	return 0;
}
