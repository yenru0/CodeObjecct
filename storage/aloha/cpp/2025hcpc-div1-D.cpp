#include <iostream>

using namespace std;
using ll = long long;

ll N;
ll ans;

void init();
void solve();

int main() {
    ios_base::sync_with_stdio(0);
    cin.tie(0); cout.tie(0);

    init();
    solve();
    cout << ans << "\n";
}

void init() {
    cin >> N;
}
void solve() {
    ll temp = 0;

    if (N % 6 == 0) {
        temp += N*(N-1)*(N-2) / 6;
        temp -= N/3;
        temp -= ((N-4)/2) * N;
        temp /= N;
        temp += (N-2)/2;
    } else if (N % 3 == 0) {
        temp += N*(N-1)*(N-2) / 6;
        temp -= N/3;
        temp -= ((N-3)/2) * N;
        temp /= N;
        temp += (N-1)/2;
    } else if (N % 2 == 0){
        temp += N*(N-1)*(N-2) / 6;
        temp -= ((N-2)/2)*N;
        temp /= N;
        temp += (N-2)/2;
    } else {
        temp += N*(N-1)*(N-2) / 6;
        temp -= ((N-1)/2) * N;
        temp /= N;
        temp += (N-1)/2;
    }

    ans = temp;
}