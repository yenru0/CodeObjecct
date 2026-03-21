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

fn get_primes(usize n) -> vector<usize> {
    let primes = vector<usize>();
    primes.push_back(2);
    for (usize i = 3; i <= n; i += 2) {
        for (let p: primes) {
            if (i % p == 0) {
                break;
            } else if (p * p > i) {
                primes.push_back(i);
                break;
            }
        }
    }
    return primes;
}

fn main() -> int {
    fastio();
    usize m, n;
    cin >> m >> n;

    let primes = get_primes(n);
    let s = lower_bound(primes.begin(), primes.end(), m);
    while (s < primes.end() && *s <= n) {
        cout << *s << "\n";
        s += 1;
    }
}