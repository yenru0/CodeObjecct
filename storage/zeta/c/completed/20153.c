#include <stdio.h>
#include <stdlib.h>

#define max(x, y) (x) > (y) ? (x) : (y)

// 영웅이 표기법 31 -> 16 + 8 + 4 + 2 + 1 11111
// 5(101) = 4 + 1, 7(111) = 4 + 2 + 1, 11 = 8 + 2 + 1 1011
// all 1001 5빼면 1100(12) 7빼면 1110(14) 당첨
// 하나 빼기
// 짝수면 제거 --> 사라지면 없음
// 홀수면 생존 --> 생기면 뭐
int 영웅이찾기(int N, int *A) {
    int all = 0;
    for (int i = 0; i < N; i++) {
        all ^= A[i];
    }

    int M = all;
    for (int i = 0; i < N; i++) {
        M = max(M, all ^ A[i]);
    }
    return M;
}

int main() {
    int power[22] = {0};
    int N;
    scanf("%d", &N);
    int *A = (int *) calloc(N, sizeof(int));
    for (int i = 0; i < N; i++) {
        scanf("%d", A + i);
    }

    int r = 영웅이찾기(N, A);
    printf("%d%d\n", r, r);
    return 0;
}