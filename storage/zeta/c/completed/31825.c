#include <stdio.h>
#include <stdlib.h>

typedef struct Alphabets {
    char *string;
    int len;
} Alphabets;

char shift(char x) {
    if (x == 90) return 65;
    else
        return x + 1;
}

void querry_shift(Alphabets *alphas, int left, int right) {
    for (int i = left - 1; i < right; i++) {
        *(alphas->string + i) = shift(*(alphas->string + i));
    }
}

int querry_string(Alphabets *alphas, int left, int right) {
    int cnt = 0;
    char head = 0, now;
    for (int i = left - 1; i < right; i++) {
        now = *(alphas->string + i);
        if (head != now) {
            head = now;
            cnt++;
        }
    }
    return cnt;
}

int querry(Alphabets *alphas, int left, int right, int type) {
    if (type == 2) {
        querry_shift(alphas, left, right);
        return -1;
    } else {
        return querry_string(alphas, left, right);
    }
}

int main() {
    int N, Q, left, right, type, ret;
    scanf("%d %d", &N, &Q);
    Alphabets alphas;
    alphas.len = N;
    alphas.string = (char *) malloc(alphas.len * sizeof(char));
    scanf("%s", alphas.string);
    for (int i = 0; i < Q; i++) {
        scanf("%d %d %d", &type, &left, &right);
        ret = querry(&alphas, left, right, type);
        if (ret < 0) {
            continue;
        } else {
            printf("%d\n", ret);
        }
    }
    free(alphas.string);
}