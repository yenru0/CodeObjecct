#include <iostream>

int main() {
    int N;
    std::string s;
    std::cin >> N;

    for (int i = 0; i < N; i++) {
        std::cin >> s;
        std::cout << s[0] << s.back() << std::endl;
    }
    return 0;
}