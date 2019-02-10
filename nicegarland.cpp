#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

int count(string input, char *seq)
{
    int res = 0;
    for(int i=0;i<input.size();i++)
        if(input[i] != seq[i%3])
            res++;
    return res;
}

int main()
{
    ios::sync_with_stdio(false);
    int n;
    string input;
    cin >> n;
    cin >> input;
    char *current;
    char winner[4];
    char possibles[][4] = {"RGB", "RBG", "GRB", "GBR", "BRG","BGR"};
    int res = 2000000000;
    for(int i=0;i<6;i++) {
        current = possibles[i];
        int current_count = count(input, current);
        if (current_count < res) {
            res = current_count;
            strcpy(winner, current);
        }
    }
    cout << res << endl;
    for(int i=0;i<input.size();i++) {
        cout << winner[i%3];
    }
    cout << endl;
    return 0;
}
