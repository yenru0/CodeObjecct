#include <stdio.h>
#include <stdlib.h>

typedef struct Honeycomb {
    int x;
    int loc;
    int compressed;
} Honeycomb;

int compare_x(const void *a, const void *b) {
    Honeycomb A = *(Honeycomb *) a;
    Honeycomb B = *(Honeycomb *) b;
    if (A.x > B.x) {
        return 1;
    } else if (A.x < B.x) {
        return -1;
    } else {
        return 0;
    }
}

int compare_loc(const void *a, const void *b) {
    Honeycomb A = *(Honeycomb *) a;
    Honeycomb B = *(Honeycomb *) b;
    if (A.loc > B.loc) {
        return 1;
    } else if (A.loc < B.loc) {
        return -1;
    } else {
        return 0;
    }
}

// 좌표 압축 알고리즘
void compress(int N, Honeycomb *arr) {
    qsort(arr, N, sizeof(Honeycomb), compare_x);

    for (int i = 1; i < N; i++) {
        if (arr[i].x == arr[i - 1].x) {
            arr[i].compressed = arr[i - 1].compressed;
        } else {
            arr[i].compressed = arr[i - 1].compressed + 1;
        }
    }

    qsort(arr, N, sizeof(Honeycomb), compare_loc);
}

// C11(Clang)에서 런타임에러: 백준 clang에서 qsort가 정렬을 제대로 하지 못하는 문제
int main() {
    int N;
    scanf("%d", &N);
    Honeycomb *arr;;
    arr = (Honeycomb *) calloc(N, sizeof(Honeycomb));
    for (int i = 0; i < N; i++) {
        scanf("%d", &arr[i].x);
        arr[i].loc = i;
        arr[i].compressed = 0;
    }

    compress(N, arr);

    for (int i = 0; i < N - 1; i++) {
        printf("%d ", arr[i].compressed);
    }
    printf("%d\n", arr[N - 1].compressed);
    free(arr);
    return 0;
}