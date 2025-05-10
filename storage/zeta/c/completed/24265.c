#include <stdio.h>

int main() {
    long long n;
    scanf("%lld", &n);
    printf("%lld\n%d\n", n * (n - 1L) / 2L, 2);
    return 0;
}