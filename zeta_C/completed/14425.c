#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int STR_LEN = 501;

int is_in(char *a, char (*S)[STR_LEN], int N) {

    for (int i = 0; i < N; i++) {
        int c = strcmp(a, S[i]);
        if (c == 0) {
            return 1;
        }
    }
    return 0;
}

int main() {
    int N, M;

    scanf("%d %d", &N, &M);

    char S[N][STR_LEN];
    char check[M][STR_LEN];

    for (int i = 0; i < N; i++) {
        scanf("%s", (S[i]));
    }

    for (int i = 0; i < M; i++) {
        scanf("%s", (check[i]));
    }

    int count = 0;
    for (int i = 0; i < M; i++) {
        count += is_in((check[i]), S, N);
    }
    printf("%d\n", count);
    return 0;
}