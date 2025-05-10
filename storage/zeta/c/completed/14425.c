#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int STR_LEN = 501;

int comp(const void *a, const void *b) {
    return strcmp((char *) a, (char *) b);
}

int is_in(char *a, char (*S)[STR_LEN], int N) {
    // binary search
    int low = 0, high = N - 1;
    while (low <= high) {
        int mid = (low + high) / 2;

        int k = (int) strcmp(a, S[mid]);
        if (k == 0) return 1;
        else if (k < 0)
            high = mid - 1;
        else if (k > 0)
            low = mid + 1;
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

    qsort(S, N, sizeof(char) * 501, comp);

    int count = 0;
    for (int i = 0; i < M; i++) {
        count += is_in((check[i]), S, N);
    }
    printf("%d\n", count);
    return 0;
}