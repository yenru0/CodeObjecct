#include <cmath>
#include <iostream>
#include <vector>

#define let auto
#define fn auto
#define usize size_t
using namespace std;

fn fastio() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
}

const let N_MAX = 1'000'000;

let Primes = vector<usize>();

fn generate_primes(usize n) {
    Primes.clear();
    Primes.push_back(2);
    for (usize i = 3; i <= n; i += 2) {
        for (let p: Primes) {
            if (i % p == 0) {
                break;
            } else if (p * p > i) {
                Primes.push_back(i);
                break;
            }
        }
    }
}

fn prime_factors(usize n) -> vector<pair<usize, usize>> {
    let factors = vector<pair<usize, usize>>();
    for (let p: Primes) {
        if (n == 1) {
            break;
        }
        if (p * p > n) {
            factors.push_back(pair(n, 1));
            break;
        }
        let cnt = 0;
        while (n % p == 0) {
            n /= p;
            cnt += 1;
        }
        if (cnt > 0) {
            factors.push_back(pair(p, cnt));
        }
    }
    return factors;
}

fn divisor_sum(usize n) -> usize {
    static usize div_sum[N_MAX + 1];
    if (div_sum[n] != 0) {
        return div_sum[n];
    }
    let s = 1;

    let fs = prime_factors(n);
    for (let f: fs) {
        s *= ((usize) pow(f.first, f.second + 1) - 1) / (f.first - 1);
    }

    div_sum[n] = s;
    return s;
}

fn cumulative_divisor_sum(usize n) -> usize {
    static usize cum_div_sum[N_MAX + 1];

    if (cum_div_sum[n] != 0) {
        return cum_div_sum[n];
    }

    for (usize i = 1; i <= N_MAX; i++) {
        cum_div_sum[i] = divisor_sum(i) + cum_div_sum[i - 1];
    }
    return cum_div_sum[n];
}

fn main() -> int {
    fastio();
    generate_primes(N_MAX);
    usize t;
    cin >> t;
    for (usize i = 0; i < t; i++) {
        usize n;
        cin >> n;
        cout << cumulative_divisor_sum(n) << "\n";
    }
}