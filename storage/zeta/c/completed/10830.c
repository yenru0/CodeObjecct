#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MATRIX_TYPE uint64_t

#define MODULO 1000

typedef struct SquareMatrix {
    uint32_t n;
    MATRIX_TYPE *data;
} SquareMatrix;

SquareMatrix *mat_create(uint32_t n) {
    SquareMatrix *mat = malloc(sizeof(SquareMatrix));
    mat->n = n;
    mat->data = calloc(n * n, sizeof(MATRIX_TYPE));
    return mat;
}

int32_t mat_copy(SquareMatrix *src, SquareMatrix *dst) {
    if (src->n != dst->n) {
        return 0;
    }
    memcpy(dst->data, src->data, sizeof(MATRIX_TYPE) * src->n * src->n);
    return 1;
}

int32_t mat_mul(SquareMatrix *a, SquareMatrix *b, SquareMatrix *dst) {
    // dst <- a * b
    if (a->n != b->n || b->n != dst->n) {
        return 0;
    }

    uint32_t n = a->n;

    for (size_t i = 0; i < n; i++) {
        for (size_t j = 0; j < n; j++) {
            MATRIX_TYPE sum = 0;
            for (size_t k = 0; k < n; k++) {
                sum += a->data[i * n + k] * b->data[k * n + j];
            }
            dst->data[i * n + j] = sum;
        }
    }
    return 1;
}

int32_t mat_mod(SquareMatrix *mat) {
    for (size_t i = 0; i < mat->n * mat->n; i++) {
        mat->data[i] %= MODULO;
    }
    return 1;
}

void mat_print(SquareMatrix *mat) {
    for (size_t i = 0; i < mat->n; i++) {
        for (size_t j = 0; j < mat->n; j++) {
            printf("%ld ", mat->data[i * mat->n + j]);
        }
        printf("\n");
    }
}

void mat_identity(SquareMatrix *mat) {
    for (size_t i = 0; i < mat->n; i++) {
        mat->data[i * mat->n + i] = 1;
    }
}

void mat_free(SquareMatrix *mat) {
    free(mat->data);
    free(mat);
}

int main() {
    uint32_t n;
    scanf("%d", &n);
    uint64_t b;
    scanf("%ld", &b);

    SquareMatrix *a = mat_create(n);

    for (size_t i = 0; i < n * n; i++) {
        scanf("%ld", a->data + i);
    }
    SquareMatrix **mats = calloc(64, sizeof(SquareMatrix *));
    mats[1] = a;
    for (int i = 2; i <= 38; i++) {
        mats[i] = mat_create(n);
        mat_mul(mats[i - 1], mats[i - 1], mats[i]);
        mat_mod(mats[i]);
    }

    SquareMatrix *res = mat_create(n);
    mat_identity(res);
    SquareMatrix *temp = mat_create(n);
    SquareMatrix *temp2;
    uint64_t bitind = 1;
    for (size_t i = 1; i <= 38; i++) {
        if ((b & bitind) != 0) {
            mat_mul(res, mats[i], temp);
            mat_mod(temp);
            temp2 = res;
            res = temp;
            temp = temp2;
        }
        bitind = bitind << 1;
    }

    mat_print(res);

    for (size_t i = 0; i < 64; i++) {
        (mats[i] == NULL) ?: mat_free(mats[i]);
    }
    free(mats);

    mat_free(res);
    mat_free(temp);

    res = NULL;
    temp = NULL;

    return 0;
}
