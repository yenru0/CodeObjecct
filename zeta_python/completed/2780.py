Mem = [[0] * 10 for i in range(1000)]


def I(l, k):
    if l == 1:
        return 1
    else:
        if Mem[l - 1][k] != 0:
            return Mem[l - 1][k]
        if k == 1:
            temp = (I(l - 1, 2) + I(l - 1, 4)) % 1234567
        elif k == 2:
            temp = (I(l - 1, 1) + I(l - 1, 3) + I(l - 1, 5)) % 1234567
        elif k == 3:
            temp = (I(l - 1, 2) + I(l - 1, 6)) % 1234567
        elif k == 4:
            temp = (I(l - 1, 1) + I(l - 1, 5) + I(l - 1, 7)) % 1234567
        elif k == 5:
            temp = (I(l - 1, 2) + I(l - 1, 4) + I(l - 1, 6) + I(l - 1, 8)) % 1234567
        elif k == 6:
            temp = (I(l - 1, 3) + I(l - 1, 5) + I(l - 1, 9)) % 1234567
        elif k == 7:
            temp = (I(l - 1, 4) + I(l - 1, 8) + I(l - 1, 0)) % 1234567
        elif k == 8:
            temp = (I(l - 1, 5) + I(l - 1, 7) + I(l - 1, 9)) % 1234567
        elif k == 9:
            temp = (I(l - 1, 6) + I(l - 1, 8)) % 1234567
        elif k == 0:
            temp = (I(l - 1, 7)) % 1234567
        Mem[l - 1][k] = temp
        return temp


T = int(input())
Ns = [int(input()) for i in range(T)]

for n in Ns:
    print(sum(I(n, i) for i in range(10))%1234567)
