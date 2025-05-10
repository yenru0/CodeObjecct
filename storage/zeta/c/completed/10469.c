#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int x, y;
} Point;

int main() {
    Point queens[64];
    int length = 0;
    char temp[8];
    Point temp_pos;
    bool flag = true;
    for (int i = 0; i < 8; i++) {
        scanf("%s", temp);
        for (int j = 0; j < 8; j++) {
            if (temp[j] == '*') {
                queens[length].x = i;
                queens[length].y = j;
                length++;
            }
        }
    }

    if (length != 8) flag = false;

    Point a, b;

    for (int i = 0; i < length; i++) {
        if (flag == false) {
            break;
        }
        a = queens[i];

        for (int j = 0; j < i; j++) {
            b = queens[j];
            if (a.x == b.x) {
                flag = false;
                break;
            } else if (a.y == b.y) {
                flag = false;
                break;
            } else if (abs(a.x - b.x) == abs(a.y - b.y)) {
                flag = false;
                break;
            }
        }
    }

    flag ? printf("valid\n") : printf("invalid\n");

    return 0;
}