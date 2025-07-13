#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

long long total_legal_minimum_length(int length, vector<int> pos) {

    long long legal_length = pos[0];

    for (auto i = 0; i < pos.size() - 1; i++) { legal_length += min(pos[i] + pos[i + 1], length - pos[i] + length - pos[i + 1]); }
    legal_length += min(length - pos[pos.size() - 1], pos[pos.size() - 1]);

    return legal_length;

}

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);

    size_t t;
    cin >> t;

    for (auto test_case = 1; test_case <= t; test_case++) {
        int n, l;
        cin >> n >> l;
        int tmp;
        vector<int> p;
        for (auto i = 0; i < n; i++) {
            cin >> tmp;
            p.push_back(tmp);
        }
        cout << "Case #" << test_case << endl;
        cout << total_legal_minimum_length(l, p) << endl;
    }
}