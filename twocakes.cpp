#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;
int arr[200002];
int range[200002];

bool cmp(int a, int b) {
    return arr[a] < arr[b];
}

int main()
{
    ios::sync_with_stdio(false);
    int n;
    cin >> n;
    for(int i=0;i<2*n;i++) {
        cin >> arr[i];
        range[i] = i;
    }
    sort(range, range+2*n, cmp);
    unsigned long long best = 0;
    int a_i = 0;
    int b_i = 0;

    for(int i=0;i<2*n;i+=2) {
        best += min(abs(range[i] - a_i) + abs(range[i+1] - b_i),
                    abs(range[i] - b_i) + abs(range[i+1] - a_i));
        a_i = range[i];
        b_i = range[i+1];
    }
    cout << best << endl;
    return 0;
}
