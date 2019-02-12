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
    int n, t;
    cin >> n >> t;
    string input, aux;
    cin >> input;
    for(int i=0;i<t;i++) {
        for(int j=0;j<(int)input.size()-1;j++) {
            if (input[j] == 'B' && input[j+1] == 'G') {
                swap(input[j], input[j+1]);
                j++;
            }
        }
    }
    cout << input << endl;
    return 0;
}
