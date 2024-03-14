#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

char translate(int c) {
    if (0 <= c && c <= 9) {
        return 48 + c;
    } else {
        return 55 + c;
    }
}

vector<char> calculate(int N, int B) {
    vector<int> S(0);
    while (N >= B) {
        S.push_back(N % B);
        N /= B;

    }
    S.push_back(N % B);
    reverse(S.begin(), S.end());
    vector<char> ST(0);
    for(auto i: S) {
        ST.push_back(translate(i));
    }
    return ST;

}

int main() {
    int N;
    int B;
    cin >> N >> B;
    vector<char> s = calculate(N, B);
    for(auto i: s) {
        cout << i;
    }
    return 0;
}