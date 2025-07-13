#include <iostream>

using namespace std;

constexpr long long MOD = 1000000007;

int POW3[100001];

int get_count_decimal_012(const string &s) {
    auto n = s.size();

    if (n == 1) { if (s[0] == '1') { return 1; } else { return 2; } }

    auto cnt = 0LL;

    // f (1 until 10^(n-1))을 구하기
    auto temp = 1LL;
    POW3[0] = temp;
    for (auto i = 1; i <= (n - 1); i++) {
        temp *= 3;
        temp %= MOD;
        POW3[i] = temp;
    }


    temp += MOD - 1;
    temp %= MOD;

    cnt += temp;
    cnt %= MOD;

    // f(10^(n-1) to int(s)) 까지 구하기
    if (s[0] == '0') {
        // pass
    } else if (s[0] == '1') {
        // pass
    } else if (s[0] == '2') {
        long long tmpx = POW3[n - 1];
        cnt += tmpx;
        cnt %= MOD;
    } else {
        long long tmpx = POW3[n - 1];
        tmpx *= 2;
        cnt += tmpx;
        cnt %= MOD;
        return static_cast<int>(cnt);
    }

    bool flag = false;
    for (int i = 1; i < n; i++) {
        auto ch = s[i];
        if (ch == '0') { continue; } else if (ch == '1') {
            long long tmpx = POW3[n - i - 1];
            cnt += tmpx;
            cnt %= MOD;
        } else if (ch == '2') {
            long long tmpx = 2 * POW3[n - i - 1];
            cnt += tmpx;
            cnt %= MOD;
        } else {
            long long tmpx;

            tmpx = POW3[n - i];

            cnt += tmpx;
            cnt %= MOD;
            flag = true;
            break;
        }
    }

    if (!flag) {
        cnt++;
        cnt %= MOD;
    }

    return static_cast<int>(cnt);
}

int main() {
    ios_base::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;
    cin >> t;
    for (auto test_case = 1; test_case <= t; test_case++) {
        string s;
        cin >> s;

        cout << "Case #" << test_case << endl;
        cout << get_count_decimal_012(s) << endl;
    }
}