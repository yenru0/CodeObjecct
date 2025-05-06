#include <iostream>

using namespace std;


int cnt1, cnt2, f[42];

int fib(int n) {
    if (n == 1 or n == 2) {
        cnt1++;
        return 1;
    } else return (fib(n - 1) + fib(n - 2));
}

int fibonacci(int n) {
    return f[n];
}

int main() {

    int n;

    cin >> n;

    fib(n);

    f[1] = 1;
    f[2] = 1;

    for (int i = 3; i <= n; i++) {
        f[i] = f[i - 1] + f[i - 2];
        cnt2++;
    }
    cout << cnt1 << " " << cnt2 << endl;

    return 0;
}