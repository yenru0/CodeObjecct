#include <stdio.h>
#include <stdlib.h>

int fib(int n) {
    if (n == 1 || n == 0) {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

int main() {
    int temp;
    scanf("%d", &temp);
    printf("%d", fib(temp));
}