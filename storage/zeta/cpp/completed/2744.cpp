#include <iostream>

using namespace std;

int main() {
    string s;
    cin >> s;
    for (int i = 0; i < s.length(); i++) {
        char c = s[i];
        if (65 <= c && c <= 90) {
            s[i] += 32;
        } else {
            s[i] -= 32;
        }
    }
    cout << s;

    return 0;
}