#include <iostream>
#include <vector>

using namespace std;

long long minimum_total_movement_length(
        int n, int reds,
        const vector<int> &pos_blue_houses, const vector<int> &pos_red_houses, const vector<int> &pos_blue_shops, const vector<int> &pos_red_shops
        ) {

    long long cost = 0;

    pos_red_houses

    return cost;
}

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);

    int t;
    cin >> t;
    for (auto test_case = 1; test_case <= t; test_case++) {
        int n, reds;

        cin >> n >> reds;
        vector<int> pos_blue_houses(n - reds), pos_red_houses(reds), pos_blue_shops(n - reds - 1), pos_red_shops(reds + 1);

        for (auto i = 0; i < n - reds; i++) { cin >> pos_blue_houses[i]; }

        for (auto i = 0; i < reds; i++) { cin >> pos_red_houses[i]; }

        for (auto i = 0; i < n - reds - 1; i++) { cin >> pos_blue_shops[i]; }

        for (auto i = 0; i < reds + 1; i++) { cin >> pos_red_shops[i]; }

        cout << "Case #" << test_case << endl;
        cout << minimum_total_movement_length(n, reds, pos_blue_houses, pos_red_houses, pos_blue_shops, pos_red_shops) << endl;
    }

}