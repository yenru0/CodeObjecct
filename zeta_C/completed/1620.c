#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAX_LENGTH 21

typedef struct _Element {
    int index;
    char name[MAX_LENGTH];
} Element;

int ecomp(const void *a, const void *b) {
    int c = strcmp((*(Element *) a).name, (*(Element *) b).name);
    return c;
}

Element binsearch_idx(Element *dict, int N, int x) {
    int start = 0;
    int end = N - 1;
    int mid;
    while (start <= end) {
        mid = (start + end) / 2;
        if (dict[mid].index == x) {
            break;
        } else if (dict[mid].index > x) {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    return dict[mid];
}

Element binsearch_str(Element *dict, int N, char *x) {
    int start = 0;
    int end = N - 1;
    int mid;
    while (start <= end) {
        mid = (start + end) / 2;
        int c = strcmp(dict[mid].name, x);
        if (c == 0) {
            break;
        } else if (c > 0) {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    return dict[mid];
}

int main() {
    int N, M;
    scanf("%d %d", &N, &M);
    Element *dictionary = (Element *) calloc(N, sizeof(Element));
    Element *str_dictionary = (Element *) calloc(N, sizeof(Element));
    for (int i = 0; i < N; i++) {
        scanf("%s", dictionary[i].name);
        dictionary[i].index = i + 1;
    }

    memmove(str_dictionary, dictionary, sizeof(Element) * N);

    qsort(str_dictionary, N, sizeof(Element), ecomp);

    for (int i = 0; i < M; i++) {
        char temp[MAX_LENGTH];
        scanf("%s", temp);
        Element e;
        if (temp[0] <= '9' && temp[0] >= '0') {
            e = binsearch_idx(dictionary, N, atoi(temp));
            printf("%s\n", e.name);
        } else {
            e = binsearch_str(str_dictionary, N, temp);
            printf("%d\n", e.index);
        }
    }

    return 0;
}