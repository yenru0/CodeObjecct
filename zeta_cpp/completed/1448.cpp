#include <algorithm>
#include <functional>
#include <iostream>

using namespace std;

int get_max(int N, int *S) {
    for (int i = 0; i < N - 2; i++) {
        if (S[i] < S[i + 1] + S[i + 2]) return S[i] + S[i + 1] + S[i + 2];
    }
    return -1;
}

int main() {

    int N;
    int S[1000000];

    cin >> N;

    for (int i = 0; i < N; i++) {
        cin >> S[i];
    }

    sort(S, S + N, greater<>());

    cout << get_max(N, S) << endl;

    return 0;
}