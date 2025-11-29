#include <iostream>

using namespace std;

int N;

void init();
void solve();
void func(int m);

int main() {
    ios_base::sync_with_stdio(0);
    cin.tie(0); cout.tie(0);

    init();
    solve();
}

void init() {
    cin >> N;
}
void solve() {
    cout << "Yes" << "\n";

    if (N % 2 == 0) {
        func(N / 2);
        
    } else if (N % 2 == 1) {
        func(N / 2);
        cout << 0;

    }
}
void func(int m) {
    int k = m / 2;

    if (m % 2 == 0) {
        if (k % 2 == 0) {
            for (int i = 1; i <= m; i++) {
                if (i % 2) cout << i << " " << -i << " ";
                else cout << -i << " " << i << " ";
            }
        } else if (k % 2 == 1) {

        }

    } else if (m % 2 == 1) {
        if (k % 2 == 0) {
            cout << -2 << " " << 1 << " ";
            for (int i = 3; i <= m+1; i++) {
                if (i % 2) cout << i << " " << -i << " ";
                else cout << -i << " " << i << " ";
            }
        } else if (k % 2 == 1) {


        }
    }
}