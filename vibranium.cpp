#include <iostream>
#include <math.h>
#include <iomanip>
using namespace std;

int main(int argc, char **argv)
{

    int tc;
    cin >> tc;
    for(int currenttc=1;currenttc<=tc;currenttc++) {
        int M, D;
        cin >> M >> D;
        double vol = M/ (double) D;
        double pi = atan(1)*4;
        double result = 4*pi*pow(3.0*vol / (4.0*pi), 2.0/3);
        cout << "Case " << currenttc << ": " << fixed << setprecision(4) << result << endl;
    }
    return 0;
}
