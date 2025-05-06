N = int(input())

Mem = [[0] * 10 for i in range(100)]


def I(l, k):
    if 0 <= k <= 9:
        if l == 1:
            return 1
        elif Mem[l-1][k] != 0:
            return Mem[l-1][k]
        else:
            temp = (I(l - 1, k - 1) + I(l - 1, k + 1)) % 1000000000
            Mem[l-1][k] = temp
            return temp
    else:
        return 0


print(sum(I(N, i) for i in range(1, 10)) % 1000000000)
