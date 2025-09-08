#include <map>
#include <iostream>

using namespace std;

void fastio() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
}

int main() {
    fastio();
    int n;
    cin >> n;
    map<int, int> 상근이카드;

    int x;
    for (auto i = 0; i < n; i++) {
        cin >> x;
        auto pos = 상근이카드.find(x);
        if (pos == 상근이카드.end()) {
            상근이카드[x] = 1;
        } else {
            pos->second++;
        }
    }

    int m;
    cin >> m;

    for (auto i = 0; i < m; i++) {
        cin >> x;
        auto pos = 상근이카드.find(x);
        if (pos == 상근이카드.end()) {
            cout << "0 ";
        } else {
            cout << pos->second << " ";
        }
    }
    cout << "\n";
}