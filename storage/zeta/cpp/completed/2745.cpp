#include <iostream>
#include <cmath>

using namespace std;

int translate(char c) {
    if (48 <= c && c <= 57) {
        return c - 48;
    } else {
        return c - 55;
    }
}

int calculate(string s, int n) {
    int l = s.length();
    int r = 0;
    for (int i = 0; i < l; i++) {
        r += translate(s[i]) * pow(n, l - i - 1);
    }
    return r;
}


int main() {
    string s;
    int n;
    cin >> s >> n;
    cout << calculate(s, n);
    return 0;
}