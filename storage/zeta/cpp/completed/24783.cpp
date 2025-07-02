#include <vector>
#include <algorithm>
#include <iostream>
#include <inttypes.h>


std::vector <uint16_t> get_possible_numbers(uint16_t a, uint16_t b) {
    std::vector <uint16_t> v;

    v.push_back(a + b);
    if (a > b) {
        v.push_back(a - b);
    } else {
        v.push_back(b - a);
    }

    if (a <= 10000 / b) {
        v.push_back(a * b);
    }

    if (a % b == 0) {
        v.push_back(a / b);
    }
    if (b % a == 0) {
        v.push_back(b / a);
    }

    return v;
}

bool is_possible_to_make(uint16_t a, uint16_t b, uint16_t c) {
    auto nums = get_possible_numbers(a, b);
    return std::find(nums.begin(), nums.end(), c) != nums.end();
}


int32_t main() {
    std::cin.tie(0);
    std::cout.tie(0);
    std::ios_base::sync_with_stdio(false);

    std::size_t t;
    std::cin >> t;
    uint16_t a, b, c;
    for (std::size_t i = 0; i < t; i++) {
        std::cin >> a >> b >> c;
        std::cout << (is_possible_to_make(a, b, c) ? "Possible" : "Impossible") << std::endl;
    }

    return 0;
}