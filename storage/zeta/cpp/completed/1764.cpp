#include <iostream>
#include <set>
#include <string>
#include <algorithm>
#include <vector>
#include <iterator>

using namespace std;

void fastio() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
}

int main() {
    fastio();

    int n, m;

    cin >> n >> m;

    auto 듣도못한사람 = new set<string>();
    auto 보도못한사람 = new set<string>();
    for (auto i = 0; i < n; i++) {
        string name;
        cin >> name;
        듣도못한사람->insert(name);


    }

    for (auto j = 0; j < m; j++) {
        string name;
        cin >> name;
        보도못한사람->insert(name);
    }

    auto 듣도보도못한사람 = new vector<string>();
    set_intersection(
            듣도못한사람->begin(), 듣도못한사람->end(),
            보도못한사람->begin(), 보도못한사람->end(),
            back_inserter(*듣도보도못한사람)
    );

    cout << 듣도보도못한사람->size() << "\n";
    for (auto name: *듣도보도못한사람) {
        cout << name << "\n";
    }

    delete 듣도못한사람;
    delete 보도못한사람;
    delete 듣도보도못한사람;

}