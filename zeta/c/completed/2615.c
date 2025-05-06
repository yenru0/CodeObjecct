#include <stdio.h>
#include <stdlib.h>

int get(int map[], int x, int y) {
    return map[19 * x + y];
}

int main() {
    int Map[19 * 19];
    int dx[] = {1, 0, 1, 1};
    int dy[] = {0, 1, 1, -1};
    int flag;
    int last_color;
    int last_pos_x, last_pos_y;
    for (int i = 0; i < 19; i++) {
        for (int j = 0; j < 19; j++) {
            scanf("%d", Map + i + j * 19);
        }
    }

    for (int x = 0; x < 19; x++) {
        for (int y = 0; y < 19; y++) {
            int now = get(Map, x, y);
            if (now == 0) {
                continue;
            } else {
                for (int d = 0; d < 4; d++) {
                    if (19 > x - dx[d] && x - dx[d] >= 0 && 19 > y - dy[d] && y - dy[d] >= 0) {
                        if (get(Map, x - dx[d], y - dy[d]) == now) {
                            flag = 0;
                            continue;
                        }
                    }
                    flag = 1;
                    for (int i = 1; i < 5; i++) {
                        if (19 > x + i * dx[d] && x + i * dx[d] >= 0 && 19 > y + i * dy[d] && y + i * dy[d] >= 0) {
                            if (get(Map, x + i * dx[d], y + i * dy[d]) != now) {
                                flag = 0;
                                break;
                            }
                        } else {
                            flag = 0;
                            break;
                        }
                    }
                    if (flag) {
                        if (19 > x + 5 * dx[d] && x + 5 * dx[d] >= 0 && 19 > y + 5 * dy[d] && y + 5 * dy[d] >= 0) {
                            if (get(Map, x + 5 * dx[d], y + 5 * dy[d]) == now) {
                                flag = 0;
                                continue;
                            }
                        }
                    }

                    if (flag) {
                        break;
                    }
                }
            }
            if (flag) {
                last_color = now;
                last_pos_x = x;
                last_pos_y = y;
                break;
            }
        }
        if (flag) {
            break;
        }
    }
    flag ? printf("%d\n%d %d\n", last_color, last_pos_y + 1, last_pos_x + 1) : printf("%d\n", 0);

    return 0;
}