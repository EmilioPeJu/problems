#include <iostream>
using namespace std;

int main(int argc, char **argv)
{
    char c;
    int i=0;
    while (true) {
        cin.get(c);
        if (cin.eof()) break;
        if (c == '"') {
            cout << (i==0?"``":"''");
            i = 1 - i;
        } else {
            cout << c;
        }
    }
    return 0;
}
