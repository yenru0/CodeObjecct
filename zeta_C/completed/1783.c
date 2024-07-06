#include <stdio.h>
#include <stdlib.h>
#include <string.h>
// M 개수에 주요한 변수를 가지고 있음. N = 1이면 무조건 1
// 1 1 1 1 1
// 1 1 1 1 1
int sick_knight(int N, int M) {
    if (N == 1) {
        return 1;
    } else if (M == 1) {
        return 1;
    } else if (M == 2) {
        if (N < 3) {
            return 1;
        } else {
            return 2;
        }
    } else if (M == 3) {
        if (N < 3) {
            return 2;
        } else {
            return 3;
        }
    } else if (M == 4) {
        if (N < 3) {
            return 2;
        } else {
            return 4;
        }
    } else if (M == 5) {
        if (N < 3) {
            return 3;
        } else {
            return 4;
        }
    } else if (M == 6) {
        if (N < 3) {
            return 3;
        } else {
            return 4;
        }
    } else if (M >= 7) {
        if (N < 3) {
            return 4;
        } else {
            return M - 2;
        }
    }
}

int main() {
    int N, M;
    scanf("%d %d", &N, &M);
    printf("%d\n", sick_knight(N, M));
    return 0;
}