#include <stdio.h>
#include <stdlib.h>

int comp(const void *a, const void *b) {
    return *(int *) a - *(int *) b;
}

int is_in(int x, int *cards, int size) {
    int begin = 0, end = size - 1, mid;
    while (begin <= end) {
        mid = (begin + end) / 2;
        if (cards[mid] == x) {
            return 1;
        } else if (cards[mid] > x) {
            end = mid - 1;
        } else {
            begin = mid + 1;
        }
    }
    return 0;
}

int main() {
    int N;
    scanf("%d", &N);

    int cards[N];
    for (int i = 0; i < N; i++) {
        scanf("%d", &cards[i]);
    }

    qsort(cards, N, sizeof(int), comp);

    int M;
    scanf("%d", &M);

    int check[M];
    for (int i = 0; i < M; i++) {
        scanf("%d", &check[i]);
    }

    for (int i = 0; i < M - 1; i++) {
        printf("%d ", is_in(check[i], cards, N));
    }
    printf("%d\n", is_in(check[M - 1], cards, N));


    return 0;
}