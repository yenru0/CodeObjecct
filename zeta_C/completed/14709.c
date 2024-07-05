#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct IntPair {
    int first;
    int second;
} IntPair;

bool fox(int N, IntPair *pairs) {
    bool _b13 = false;
    bool _b14 = false;
    bool _b34 = false;
    for (int i = 0; i < N; i++) {
        if (pairs[i].first == 2 || pairs[i].first == 5 || pairs[i].second == 2 || pairs[i].second == 5) return false;
        if (pairs[i].first == 1) {
            if (pairs[i].second == 3) {
                _b13 = true;
            } else if (pairs[i].second == 4) {
                _b14 = true;
            }
        } else if (pairs[i].first == 3) {
            if (pairs[i].second == 1) {
                _b13 = true;
            } else if (pairs[i].second == 4) {
                _b34 = true;
            }
        } else if (pairs[i].first == 4) {
            if (pairs[i].second == 1) {
                _b14 = true;
            } else if (pairs[i].second == 3) {
                _b34 = true;
            }
        }
    }
    return _b13 && _b14 && _b34;
}

int main(void) {
    int N;
    scanf("%d", &N);
    IntPair *pairs = (int *) calloc(N, sizeof(IntPair));
    for (int i = 0; i < N; i++) {
        scanf("%d %d", &pairs[i].first, &pairs[i].second);
    }
    bool isFox = fox(N, pairs);
    isFox ? printf("Wa-pa-pa-pa-pa-pa-pow!\n") : printf("Woof-meow-tweet-squeek\n");
    return 0;
}