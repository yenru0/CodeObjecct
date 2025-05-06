#include <math.h>
#include <stdio.h>
#include <stdlib.h>

int compare(const void *first, const void *second) {
    if (*(int *) first < *(int *) second)
        return 1;
    else if (*(int *) first > *(int *) second)
        return -1;
    else
        return 0;
}

int main() {
    int T;
    scanf("%d", &T);
    int N, X, Y;
    int *V;
    for (int i = 0; i < T; i++) {
        scanf("%d %d %d", &N, &X, &Y);
        V = (int *) calloc(N, sizeof(int));
        for (int j = 0; j < N; j++) {
            scanf("%d", &V[j]);
        }
        int my_speed = V[N - 1];
        qsort(V, N - 1, sizeof(int), compare);
        int rival_speed = V[0];
        double rival_time;
        if (my_speed > rival_speed) {
            printf("%d\n", 0);
            free(V);
            continue;
        } else {
            rival_time = (double) X / (double) rival_speed;
        }
        double Z;
        int iZ;
        if (rival_time >= 1) {
            Z = X - (rival_time - 1) * my_speed;
        } else {
            Z = rival_speed + 1;
        }

        iZ = floor(Z + 1);
        if (iZ > Y) {
            printf("%d\n", -1);
        } else {
            printf("%d\n", iZ);
        }
        free(V);
    }

    return 0;
}