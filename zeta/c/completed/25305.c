#include <stdio.h>

int main() {
    int N, k;

    scanf("%d %d\n", &N, &k);

    int 배열[N + 1];
    int 임시;
    for (int i = 0; i < N; i++) {
        scanf("%d", &배열[i]);
    }

    for (int i = 0; i < N - 1; i++) {
        for (int j = 0; j < N - 1; j++) {
            if (배열[j] < 배열[j + 1]) {
                임시 = 배열[j];
                배열[j] = 배열[j + 1];
                배열[j + 1] = 임시;
            }
        }
    }

    printf("%d\n", 배열[k - 1]);


}