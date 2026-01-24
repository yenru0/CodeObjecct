#include <iostream>
#include <vector>

#define let auto

typedef int i32;
typedef int64_t i64;
typedef size_t usize;

using namespace std;

let fastio() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
}

// for disjoints

let root(vector<usize> &parents, usize x) -> usize {
    let node = x;
    let par = parents[x];
    if (node == par) {
        return node;
    }
    parents[x] = root(parents, par);
    return parents[x];
}

let make_bridge(vector<usize> &parents, vector<usize> &weights, usize a) -> bool {
    // find representative
    let r_curr = root(parents, a);
    let r_next = root(parents, a + 1);

    parents[r_next] = r_curr;
    weights[r_curr] += weights[r_next];
    weights[r_next] = 0;
    return true;
}

// util
let tcount(usize x) -> usize {
    return x * (x - 1) / 2;
}

let cost(usize x) -> usize {
    return x * (x - 1) * (x + 1) / 6;
}

let main() -> i32 {
    usize n;

    cin >> n;

    let order_bridges = vector<usize>(n - 1);

    for (usize i = 0; i < n - 1; i++) {
        usize temp;
        cin >> temp;
        order_bridges[i] = temp - 1;
    }
    // prep for disjoint
    let parents = vector<usize>(n);
    let weights = vector<usize>(n, {1});
    for (usize i = 0; i < n; i++) {
        parents[i] = i;
    }

    let tot_cnt = (usize) 0;
    let tot_cost = (usize) 0;
    // construction starts
    for (let br: order_bridges) {
        let rt = root(parents, br);
        let rtb = root(parents, br + 1);
        let subed_cost = cost(weights[rt]) + cost(weights[rtb]);
        let subed_cnt = tcount(weights[rt]) + tcount(weights[rtb]);
        make_bridge(parents, weights, br);
        let added_cost = cost(weights[rt]) + cost(weights[rtb]);
        let added_cnt = tcount(weights[rt]) + tcount(weights[rtb]);
        tot_cnt += added_cnt - subed_cnt;
        tot_cost += added_cost - subed_cost;
        cout << tot_cnt << " " << tot_cost << "\n";
    }
}
