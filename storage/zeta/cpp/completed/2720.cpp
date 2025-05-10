#include <iostream>

using namespace std;

void solve(int x, int c[], int cat[]) {
    for (int i = 0; i < 4; i++) {
        c[i] = (x / cat[i]);
        x %= cat[i];
    }
}

int main() {
    int cat[4] = {25, 10, 5, 1};

    int N;

    cin >> N;
    for (int i = 0; i < N; i++) {
        int x;
        int coins[4];
        cin >> x;
        solve(x, coins, cat);
        for (int j = 0; j < 4; j++) {
            cout << coins[j] << " ";
        }
        cout << endl;
    }
    return 0;
}