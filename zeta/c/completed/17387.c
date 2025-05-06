#include <stdio.h>
#include <stdlib.h>

typedef long long LL;

int ccw(int x1, int y1, int x2, int y2, int x3, int y3) {
    LL s = (LL) (x2 - x1) * (LL) (y3 - y1) - (LL) (y2 - y1) * (LL) (x3 - x1);
    if (s > 0) return 1;
    if (s < 0) return -1;
    return 0;
}

int compare_leq(int x1, int y1, int x2, int y2) {
    if (x1 == x2) {
        return y1 <= y2;
    }
    return x1 <= x2;
}

int isIntersect(int x1, int y1, int x2, int y2, int x3, int y3, int x4, int y4) {
    int d1 = ccw(x1, y1, x2, y2, x3, y3);
    int d2 = ccw(x1, y1, x2, y2, x4, y4);
    int d3 = ccw(x3, y3, x4, y4, x1, y1);
    int d4 = ccw(x3, y3, x4, y4, x2, y2);

    int c1 = d1 * d2;
    int c2 = d3 * d4;

    if (c1 == 0 && c2 == 0) {
        if (compare_leq(x2, y2, x1, y1)) {
            int temp = x1;
            x1 = x2;
            x2 = temp;
            temp = y1;
            y1 = y2;
            y2 = temp;
        }
        if (compare_leq(x4, y4, x3, y3)) {
            int temp = x3;
            x3 = x4;
            x4 = temp;
            temp = y3;
            y3 = y4;
            y4 = temp;
        }
        return (compare_leq(x1, y1, x4, y4) &&
                compare_leq(x3, y3, x2, y2));
    }

    return c1 <= 0 && c2 <= 0;
}

int main() {
    int x1, y1, x2, y2;
    int x3, y3, x4, y4;
    scanf("%d %d %d %d", &x1, &y1, &x2, &y2);
    scanf("%d %d %d %d", &x3, &y3, &x4, &y4);

    printf("%d\n", isIntersect(x1, y1, x2, y2, x3, y3, x4, y4));
    return 0;
}