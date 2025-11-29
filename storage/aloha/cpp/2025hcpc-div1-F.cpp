#include <malloc.h>
#include <stdint.h>
#include <stdio.h>

int main() {
    uint64_t n;

    scanf("%d", &n);
    double p = (n - 1) * (n) / (double) 4;
    printf("%f\n", p);
}