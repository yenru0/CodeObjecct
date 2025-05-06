#include <stdbool.h>
#include <stdio.h>

typedef struct TitleCondition {
    char title[12];
    int condition;
} TitleCondition;

TitleCondition total[100000];
int N, M;

// tc[i-1] < x <= tc[i]일 때, tc[i]
TitleCondition *get_title(int x) {
    int start = -1, end = N;
    int mid;

    while (start + 1 < end) {
        mid = (start + end) / 2;
        if (total[mid].condition < x) {
            start = mid;
        } else {
            end = mid;
        }
    }

    return total + end;
}

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 0; i < N; i++) {
        scanf("%s %d", &total[i].title, &total[i].condition);
    }
    int temp;
    for (int i = 0; i < M; i++) {
        scanf("%d", &temp);
        printf("%s\n", get_title(temp)->title);
    }

    return 0;
}