#include <iostream>

using namespace std;

int main() {

    ios_base::sync_with_stdio(0);
    cin.tie(nullptr);
    int T;
    int A, B;
    std::cin >> T;

    for (int i = 0; i < T; i++) {
        std::cin >> A >> B;
        std::cout << A + B << '\n';
    }

    return 0;
}