#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 1000000

typedef struct {
    int x;
    int y;
} Position;

Position stack[MAX_SIZE];
int top = -1;

void push(Position item) {
    stack[++top] = item;
}

Position pop() {
    return stack[top--];
}

int is_empty() {
    if (top == -1) {
        return 1;
    } else {
        return 0;
    }
}

int main() {
    int N;
    scanf("%d", &N);
    int **A = (int **) calloc(N, sizeof(int *));
    A[0] = (int *) calloc(N * N, sizeof(int));
    for (int i = 1; i < N; i++) {
        A[i] = A[i - 1] + N;
    }

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            scanf("%d", &A[i][j]);
        }
    }

    int **V = (int **) calloc(N, sizeof(int *));
    V[0] = (int *) calloc(N * N, sizeof(int));
    for (int i = 1; i < N; i++) {
        V[i] = V[i - 1] + N;
    }

    int flag = 0;

    Position nowainit;
    nowainit.x = 0;
    nowainit.y = 0;

    push(nowainit);

    Position now;
    Position target;
    int amount;
    while (!is_empty()) {
        now = pop();
        amount = A[now.x][now.y];
        if (V[now.x][now.y]) {
            continue;
        }
        V[now.x][now.y] = 1;

        if (amount == 0) {
            continue;
        } else if (amount == -1) {
            flag = 1;
            break;
        }

        if (now.x + amount >= N) {

        } else {
            target.x = now.x + amount;
            target.y = now.y;
            push(target);
        }

        if (now.y + amount >= N) {

        } else {
            target.x = now.x;
            target.y = now.y + amount;
            push(target);
        }
    }
    if (flag) printf("HaruHaru");
    else
        printf("Hing");

    free(A[0]);
    free(A);
    free(V[0]);
    free(V);
    return 0;
}