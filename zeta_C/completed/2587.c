#include <stdio.h>


int main() {
    int 배열[5];
    int 합 = 0;
    int 임시;

    scanf("%d\n%d\n%d\n%d\n%d", &배열[0], &배열[1], &배열[2], &배열[3], &배열[4]);

    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < 4; j++) {
            if (배열[j] > 배열[j + 1]) {
                임시 = 배열[j];
                배열[j] = 배열[j + 1];
                배열[j + 1] = 임시;
            }
        }
    }

    for (int i = 0; i < 5; i++) {
        합 += 배열[i];
    }

    printf("%d\n", 합 / 5);
    printf("%d\n", 배열[2]);


    return 0;
}