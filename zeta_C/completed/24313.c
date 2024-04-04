#include <stdio.h>

int main() {
    int a1, a0, c, n0;
    scanf("%d %d\n%d\n%d", &a1, &a0, &c, &n0);

    int cond = 1;

    for (int i = n0; i <= 100; i++) {
        if (a1 * i + a0 > c * i) {
            cond = 0;
            break;
        }
    }

    printf("%d\n", cond);
    return 0;
}