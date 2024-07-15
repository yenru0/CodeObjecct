#include <stdio.h>
#include <stdlib.h>

int main() {
    int N, M, J, cnt = 0, tmp, now = 1, flag;
    scanf("%d %d", &N, &M);
    scanf("%d", &J);
    int *pos = (int *) calloc(J, sizeof(int));
    for (int i = 0; i < J; i++) {
        scanf("%d", pos + i);
    }
    for (int i = 0; i < J; i++) {
        flag = 1;
        while (flag) {
            if (now <= pos[i] && pos[i] <= now + M - 1) {
                flag = 0;
            } else if (now > pos[i]) {
                now--;
                cnt++;
            } else {
                now++;
                cnt++;
            }
        }
    }
    printf("%d\n", cnt);
}