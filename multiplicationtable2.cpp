#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef pair<int, int> ii;
typedef tuple<int, int, int> iii;
typedef vector<ii > vii;
typedef vector<iii > viii;

uint32_t result = 0;
uint32_t n, x;
vector<uint32_t> divisors;
vector<uint32_t> order;

void count_all(uint32_t acc, uint32_t step) {
    if (acc > n) return;

    if (step == divisors.size()) {
        uint32_t other = x / acc;
        if (other <=n) {
            result++;
        }
        return;
    }
    uint32_t current = divisors[step];
    for(uint32_t i=0;i<order[step];i++) {
        count_all(acc*current, step+1);
        current*=divisors[step];
    }
    count_all(acc, step+1);
}

int main()
{
    ios::sync_with_stdio(false);
    cin >> n >> x;
    uint32_t candidate = 2;
    uint32_t _x = x;
    while (_x>1) {
        if (_x%candidate==0) {
            divisors.push_back(candidate);
            order.push_back(1);
            _x/=candidate;
        }
        while (_x%candidate == 0) {
            order.back()++;
            _x /= candidate;
        }
        candidate++;
    }
    count_all(1, 0);
    cout << result << endl;
    return 0;
}
