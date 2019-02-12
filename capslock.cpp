#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

bool is_error(string input) {
    assert(input.size() > 1);
    for(int i=1;i<(int)input.size();i++) {
        char c = input[i];
        if (not (c >= 'A' && c <= 'Z'))
            return false;
    }
    return true;
}
char toggle(char c) {
    if (c >= 'A' && c <= 'Z')
        return tolower(c);
    if (c >= 'a' && c <= 'z')
        return toupper(c);
    return c;
}
int main()
{
    ios::sync_with_stdio(false);
    string input;
    cin >> input;
    if (input.size() == 1) {
        cout << (char) toggle(input[0]) << endl;
        return 0;
    }
    if (is_error(input)) {
        transform(input.begin()+1, input.end(), input.begin()+1,
                [](char c) -> char { return tolower(c); });
        input[0] = toggle(input[0]);
    }
    cout << input << endl;

    return 0;
}
