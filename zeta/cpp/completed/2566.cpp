#include<iostream>

int main() {
    int TM = 0;
    int now;
    int idx_x, idx_y;
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            std::cin >> now;
            if (now >= TM) {
                TM = now;
                idx_x = i + 1;
                idx_y = j + 1;
            }
        }

    }

    std::cout << TM << std::endl;
    std::cout << idx_x << " " << idx_y << std::endl;
}