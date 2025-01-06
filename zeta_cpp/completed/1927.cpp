#include <iostream>
#include <queue>

int main() {
    std::cin.tie(0);
    std::cout.tie(0);
    std::ios_base::sync_with_stdio(0);
    std::priority_queue<unsigned int, std::vector<unsigned int>, std::greater<int>> q;
    int N;
    std::cin >> N;
    for (int i = 0; i < N; i++) {
        int x;
        std::cin >> x;
        if (x != 0) {
            q.push(x);
        } else {
            if (q.size() != 0) {
                std::cout << q.top() << "\n";
                q.pop();
            } else {
                std::cout << 0 << "\n";
            }
        }
    }

    return 0;
}