#include <iostream>

#include <stddef.h>
#include <stdint.h>
#include <vector>

using namespace std;

int main() {
    size_t n;

    cin >> n;

    vector<uint64_t> tastes;
    uint64_t temp;
    for (size_t i = 0; i < n; i++) {
        cin >> temp;
        tastes.push_back(temp);
    }

    // pop_front
    // pop_back
    // push_back
    // push_front
}