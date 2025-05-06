#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct _Item {
    char name[6];
    char isin;
} Item;

int comp(const void *a, const void *b) {
    int cmp = strcmp((*(Item *) a).name, (*(Item *) b).name);

    return -cmp;
}

int main() {
    int N;

    scanf("%d", &N);
    Item res[N + 1];
    for (int i = 0; i < N; i++) {
        char temp[6];
        scanf("%s %s", res[i].name, temp);
        if (temp[0] == 'e') {
            res[i].isin = 1;
        } else {
            res[i].isin = 0;
        }
    }

    qsort(res, N, sizeof(Item), comp);
    for (int i = 0; i < N; i++) {
        if (comp(&res[i], &res[i + 1]) == 0) {
            i++;
        } else {
            printf("%s\n", res[i].name);
        }
    }
    return 0;
}