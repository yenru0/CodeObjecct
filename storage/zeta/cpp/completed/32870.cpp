#include <iostream>
#include <utility>

#define let auto
#define fn auto
#define usize size_t
using namespace std;

const usize MAX_N = 300'000;

fn fastio() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
}

fn main() -> int {
    fastio();
    let exists = new bool[MAX_N + 1]{false};
    let accum_minima = new usize[MAX_N + 2];
    let accum_maxima = new usize[MAX_N + 2];
    fill_n(accum_minima, MAX_N + 2, __UINT64_MAX__);
    usize n, q;
    cin >> n >> q;
    for (usize i = 0; i < n; i++) {
        usize tmp;
        cin >> tmp;
        exists[tmp] = true;
    }

    for (usize i = 1; i < MAX_N + 2; i++) {// maximum elem that is less than i
        accum_maxima[i] = (exists[i - 1] ? i - 1 : accum_maxima[i - 1]);
    }

    for (int64_t i = MAX_N; i >= 0; --i) {// minimum elem that is greater or equal than i
        accum_minima[i] = (exists[i] ? i : accum_minima[i + 1]);
    }

    let vis = new bool[MAX_N + 1];
    let lo_query = new usize[MAX_N + 1];
    let hi_query = new usize[MAX_N + 1];

    for (usize ic = 0; ic < q; ic++) {
        usize m;
        cin >> m;

        if (m == 1) {
            cout << "0 0\n";
            continue;
        } else if (vis[m]) {
            cout << lo_query[m] << " " << hi_query[m] << "\n";
            continue;
        }
        vis[m] = true;
        usize minima = m - 1;
        usize maxima = 0;
        usize i;
        for (i = 0; i <= MAX_N; i += m) {
            let key = i;
            let t_min = m - 1;
            if (accum_minima[key] != __UINT64_MAX__) {
                t_min = accum_minima[key] % m;
            }
            let t_max = accum_maxima[key] % m;
            minima = min(minima, t_min);
            maxima = max(maxima, t_max);
        }
        if (i > MAX_N) {
            maxima = max(maxima, accum_maxima[MAX_N + 1] % m);
        }
        lo_query[m] = minima;
        hi_query[m] = maxima;
        cout << minima << " " << maxima << "\n";
    }

    // free
    delete[] exists;
    delete[] accum_minima;
    delete[] accum_maxima;

    delete[] vis;
    delete[] lo_query;
    delete[] hi_query;
}