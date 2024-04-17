#include <stdio.h>
#include <stdlib.h>

int main() {
    int N, M;

    scanf("%d %d", &N, &M);

    char S[N][501];
    char check[M][501];

    for (int i = 0; i < N; i++) {
        scanf("%s", &S[i]);
    }

    for (int i = 0; i < M; i++) {
        scanf("%s", &check[i]);
    }

    return 0;
}