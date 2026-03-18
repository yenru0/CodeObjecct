#include <cmath>
#include <iostream>
#include <vector>

#define let auto
#define fn auto
#define usize size_t

using namespace std;

const usize MOD = 1000000007;

fn fastio() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
}

fn well_known_sum_draft(usize n, usize m) -> usize {
    let res = (usize) 0;
    for (usize i = 1; i <= n; i++) {
        res += (n / i) * (i % m);
        res %= MOD;
    }
    return res;
}

fn well_known_subarr_sum(usize d, usize lo, usize hi /* exclusive */, usize m) -> usize {
    let res = (usize) 0;

    let lo_margin = lo % m;
    let hi_margin = hi % m;
    if (lo / m == hi / m) {
        res += ((hi_margin * (hi_margin - 1) / 2) - (lo_margin * (lo_margin - 1) / 2)) * d;
        res %= MOD;
    } else {
        res += (hi_margin * (hi_margin - 1) / 2) * d;
        res %= MOD;

        res += (m * (m - 1) / 2 - lo_margin * (lo_margin - 1) / 2) * d;
        res %= MOD;

        let d_cycle = (d * (m * (m - 1) / 2) % MOD) % MOD;
        let cycle_cnt = (hi - lo - (hi_margin + m - lo_margin)) / m;
        res += cycle_cnt * d_cycle;
        res %= MOD;
    }

    return res;
}

fn well_known_sum(usize n, usize m) -> usize {
    let res = (usize) 0;
    let steps = vector<usize>();
    steps.push_back(n + 1);
    let k = (usize) pow(n, 0.5);
    for (usize i = 2; i <= k + 1; i++) {
        steps.push_back(n / i + 1);
    }
    let step_size = steps.size();
    for (usize i = 1; i < steps[step_size - 1]; i++) {
        res += (n / i) * (i % m);
        res %= MOD;
    }

    for (usize i = 0; i < step_size - 1; i++) {
        let temp = well_known_subarr_sum(i + 1, steps[i + 1], steps[i], m);
        res += temp;
        res %= MOD;
    }

    return res;
}

fn main() -> int {
    fastio();
    usize n;
    usize m;

    cin >> n;
    cin >> m;

    if (n <= 10) {
        cout << well_known_sum_draft(n, m) << endl;
    } else if (m == 1) {
        cout << 0 << endl;
    } else {
        cout << well_known_sum(n, m) << endl;
    }
    return 0;
}