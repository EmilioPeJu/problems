#include <iostream>
using namespace std;

int main(int argc, char **argv)
{
    int tc;
    cin >> tc;
    while(tc--) {
        int n, a, b, c, d;
        cin >> n >> a >> b >> c >> d;
        if (a+b+c+d > n/2) cout << "Yes" << endl;
        else  cout << "No" << endl;
    }
    return 0;
}
