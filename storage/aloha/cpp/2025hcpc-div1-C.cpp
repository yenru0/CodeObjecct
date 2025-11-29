#define MAX_N 200'000
#include <iostream>

using namespace std;
using ll = long long;

ll ans;
int N, M;
ll sum[MAX_N+1][26];
string S, T;

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
    cin >> N >> M;
    cin >> S >> T;
}
void solve() {
    for (int i = 0; i < S.size(); i++) {
        char c = S[i];
        for (int j = 0; j < 26; j++) sum[i+1][j] = sum[i][j];
        sum[i+1][c-'a']++;
    }

    for (int i = 0; i < T.size(); i++) {
        char c = T[i];
        ans += sum[S.size()-T.size()+i + 1][c-'a'] - sum[i][c-'a'];
    }
}