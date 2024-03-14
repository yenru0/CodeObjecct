#include <iostream>
#include <cmath>

int main() {
    int n;
    std::cin >> n;
    std::cout << (int) pow(((1 << n) + 1), 2);
    return 0;
}