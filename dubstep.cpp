#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int get_end(string &input, int start)
{
    int index = start;
    while (index < input.size()-2) {
        if (input[index] == 'W' && input[index+1] == 'U'
                && input[index+2] == 'B')
            index+=3;
        else
            break;
    }
    return index;
}

int main()
{
    ios::sync_with_stdio(false);
    string input;
    cin >> input;
    int index = 0;
    while(input.find("WUB", index) == index)
        index += 3;
    bool wub = false;
    while (index<input.size()) {
        wub = false;
        while(input.find("WUB", index) == index) {
            index += 3;
            wub = true;
        }
        if (index < input.size()) {
            if (wub)
                cout << " ";
            else
                cout << input[index++];
        }
    }
    cout << endl;

    return 0;
}
