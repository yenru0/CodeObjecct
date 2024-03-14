#include <iostream>
#include <iomanip>

using namespace std;

int main() {
    string s;
    cin >> s;
    float v;
    if (s == "A+") {
        v = 4.3;
    } else if (s == "A0") {
        v = 4.0;
    } else if (s == "A-") {
        v = 3.7;
    } else if (s == "B+") {
        v = 3.3;
    } else if (s == "B0") {
        v = 3.0;
    } else if (s == "B-") {
        v = 2.7;
    } else if (s == "C+") {
        v = 2.3;
    } else if (s == "C0") {
        v = 2.0;
    } else if (s == "C-") {
        v = 1.7;
    } else if (s == "D+") {
        v = 1.3;
    } else if (s == "D0") {
        v = 1.0;
    } else if (s == "D-") {
        v = 0.7;
    } else if (s == "F")
        v = 0.0;
    else {
        v = 0.0;
    }
    cout << fixed << std::setprecision(1) << v;

    return 0;
}