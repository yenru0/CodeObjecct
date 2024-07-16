#include <stdio.h>
#include <stdlib.h>

int max3(int x, int y, int z) {
    if (x > y)
        if (x > z)
            return x;
        else
            return z;
    else if (y > z)
        return y;
    else
        return z;
}

int comp(const void *a, const void *b) {
    return (*(int *) b - *(int *) a);
}

int main() {
    int N, M, temp = 0;
    scanf("%d %d", &N, &M);
    int *scores = (int *) calloc(N * M, sizeof(int));
    int *maxes = (int *) calloc(M * M * M, sizeof(int));
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            scanf("%d", scores + M * i + j);
        }
    }
    for (int i = 0; i < M; i++) {
        for (int j = 0; j < M; j++) {
            if (j == i) continue;
            for (int k = 0; k < M; k++) {
                if (k == j || k == i) continue;
                temp = 0;
                for (int n = 0; n < N; n++) {
                    temp += max3(scores[M * n + i], scores[M * n + j], scores[M * n + k]);
                }
                maxes[M * M * i + M * j + k] = temp;
            }
        }
    }

    qsort(maxes, M * M * M, sizeof(int), comp);

    printf("%d", maxes[0]);
    free(scores);
    free(maxes);

    return 0;
}