#include <stdio.h>
#include <stdlib.h>

void solve(int *arr, int n, int k) {
    int *temparr = calloc(n, sizeof(int));
    if (temparr == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        return;
    }
    int cnt_longlive = 0;
    int cnt;
    int idx = -1;
    for (int i = 0; i < n; i++) {
        cnt = 0;
        while (cnt < k) {
            idx = (idx + 1) % n;
            if (temparr[idx] == 0) {
                cnt++;
            }
        }
        temparr[idx] = 1;
        arr[cnt_longlive++] = idx + 1;
    }
    free(temparr);
}

void print_result(int *arr, int n) {
    printf("<");
    for (int i = 0; i < n; i++) {
        printf("%d", arr[i]);
        if (i != n - 1) {
            printf(", ");
        }
    }
    printf(">\n");
}

int main() {
    int n, k;

    scanf("%d %d", &n, &k);

    int *arr = calloc(n, sizeof(int));
    if (arr == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    solve(arr, n, k);
    print_result(arr, n);
    free(arr);
    return 0;
}