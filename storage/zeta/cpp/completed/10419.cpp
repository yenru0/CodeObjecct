#include <iostream>

int max_late_time(int d) {
    int s = 1;
    while (s * s + s <= d) {
        s++;
    }
    return s - 1;
}

int main() {
    int T;
    std::cin >> T;
    int d;
    for (int t = 0; t < T; t++) {
        std::cin >> d;
        std::cout << max_late_time(d) << std::endl;
    }
    return 0;
}