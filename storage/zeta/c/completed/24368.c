#include <stdio.h>

int check(a2, a1, a0, c, n0) {
    if (c - a2 < 0) {
        return 0;
    } else if (c - a2 == 0) {
        if (a1 == 0) {
            if (-a0 >= 0) {
                return 1;
            }
        } else if (-a1 > 0) {
            if ((-a1) * n0 - a0 >= 0) {
                return 1;
            }
        }
        return 0;
    } else {
        int p = (c - a2) * n0 * n0 - a1 * n0 - a0;
        if (p >= 0) {
            int d = a1 * a1 + 4 * (c - a2) * a0;
            if (d <= 0) {
                return 1;
            } else {
                double axis = (double) (a1) / (2.0 * (double) (c - a2));
                if (n0 >= axis) {
                    return 1;
                }
            }
        }
        return 0;
    }
}

int main() {
    int a2, a1, a0, c, n0;
    scanf("%d %d %d", &a2, &a1, &a0);
    scanf("%d", &c);
    scanf("%d", &n0);
    printf("%d\n", check(a2, a1, a0, c, n0));
}