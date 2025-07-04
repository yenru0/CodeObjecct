#include <iostream>
#include <cinttypes>
#include <utility>
#include <vector>
#include <algorithm>
#include <queue>

using namespace std;

uint32_t get_score(const pair<int32_t, int32_t> &a, const pair<int32_t, int32_t> &b) {
    return (a.first - b.first) * (a.first - b.first) + (a.second - b.second) * (a.second - b.second);
}


uint32_t get_maximum_score(
        size_t n, size_t m,
        vector<pair<int32_t, int32_t>> pre_targets,
        queue<pair<int32_t, int32_t>> next_targets
) {
    // pq로 관리하면 더 좋을듯
    uint32_t score = 0;
    pair<int32_t, int32_t> now{0, 0};

    vector<pair<int32_t, int32_t>> targets = std::move(pre_targets);

    for (size_t i = 0; i < m; i++) {
        auto acc = max_element(targets.begin(), targets.end(),
                               [now](
                                       pair<int32_t, int32_t> const &a, pair<int32_t, int32_t> const &b
                               ) {
                                   return get_score(now, a) < get_score(now, b);
                               }
        );

        score += get_score(now, *acc);
        now = *acc;

        targets.erase(acc);
        targets.push_back(next_targets.front());
        next_targets.pop();
    }

    return score;
}


int32_t main() {
    cin.tie(nullptr);
    cout.tie(nullptr);
    ios_base::sync_with_stdio(false);

    size_t n, m;
    cin >> n >> m;


    vector<
            pair<int32_t, int32_t>
    > pre_targets;

    for (size_t i = 0; i < n; i++) {
        pair<int32_t, int32_t> p;
        cin >> p.first >> p.second;
        pre_targets.push_back(p);
    }

    queue<
            pair<int32_t, int32_t>
    >
            next_targets;

    for (size_t i = 0; i < m; i++) {
        pair<int32_t, int32_t> p;
        cin >> p.first >> p.second;
        next_targets.push(p);
    }

    cout << get_maximum_score(n, m, pre_targets, next_targets) << endl;


    return 0;
}