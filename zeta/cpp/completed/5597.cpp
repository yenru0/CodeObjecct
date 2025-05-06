#include <iostream>

int main() {
    int *arr = new int[30]{0};
    int index;
    for (int i = 0; i < 30; i++) {
        std::cin >> index;
        arr[index - 1] = 1;
    }

        for (int i = 0; i < 30; i++) {
        if (!arr[i]) {
            std::cout << i + 1 << std::endl;
        }
    }

    return 0;
}