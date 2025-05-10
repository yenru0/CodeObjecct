N = int(input())

Mem = [[[-1] * 5 for _ in range(0, 10)] for _ in range(N)]


def I(l, k, b):
    if b > 2 or b < -2:
        return 0
    if l == 0:
        return 1
    if Mem[l - 1][k][b] != -1:
        return Mem[l - 1][k][b]

    re = 0
    if 0 < k:
        re += I(l - 1, k - 1, b - 1 if b < 0 else -1) % 1000000007
    if k < 9:
        re += I(l - 1, k + 1, b + 1 if b > 0 else 1) % 1000000007
    Mem[l - 1][k][b] = re
    return re


print(sum(I(N - 1, i, 0) for i in range(0, 10)) % 1000000007)
