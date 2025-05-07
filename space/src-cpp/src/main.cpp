#include <iostream>

int fib(int n) {
    if (n == 0 || n == 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

int main() {
    int temp;
    std::cin >> temp;

    std::cout << fib(temp) << std::endl;

    return 0;
}