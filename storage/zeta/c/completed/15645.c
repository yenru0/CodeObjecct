#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
    uint32_t first;
    uint32_t second;
    uint32_t third;
} Row;

inline uint32_t max2(uint32_t a, uint32_t b) {
    return a > b ? a : b;
}

inline uint32_t max3(uint32_t a, uint32_t b, uint32_t c) {
    return max2(max2(a, b), c);
}

inline uint32_t min2(uint32_t a, uint32_t b) {
    return a < b ? a : b;
}

inline uint32_t min3(uint32_t a, uint32_t b, uint32_t c) {
    return min2(min2(a, b), c);
}

int32_t main() {
    size_t n;
    scanf("%zu", &n);

    Row row = {0, 0, 0};
    Row curr_min = {0, 0, 0};
    Row next_min = {0, 0, 0};
    Row curr_max = {0, 0, 0};
    Row next_max = {0, 0, 0};

    for (size_t i = 0; i < n; i++) {
        scanf("%d %d %d", &row.first, &row.second, &row.third);

        next_max.first = max2(curr_max.first, curr_max.second) + row.first;
        next_max.second = max3(curr_max.first, curr_max.second, curr_max.third) + row.second;
        next_max.third = max2(curr_max.second, curr_max.third) + row.third;
        curr_max = next_max;

        next_min.first = min2(curr_min.first, curr_min.second) + row.first;
        next_min.second = min3(curr_min.first, curr_min.second, curr_min.third) + row.second;
        next_min.third = min2(curr_min.second, curr_min.third) + row.third;
        curr_min = next_min;
    }

    printf("%d %d\n",
           max3(curr_max.first, curr_max.second, curr_max.third),
           min3(curr_min.first, curr_min.second, curr_min.third));
    fflush(stdout);

    return 0;
}