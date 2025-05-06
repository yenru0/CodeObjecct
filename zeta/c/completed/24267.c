#include <stdio.h>

int main() {
    long long n;
    scanf("%lld", &n);
    printf("%lld\n%d\n", n * (n - 1L) * (n - 2L) / 6L, 3);
    return 0;
}