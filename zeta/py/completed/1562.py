N = int(input())

D = [[0] * (1 << 10) for _ in range(10)]

for i in range(1, 10):
    D[i][1 << i] = 1

for i in range(0, N - 1):
    Dn = [[0] * (1 << 10) for _ in range(10)]
    for n in range(10):
        for m in range(1024):
            if n < 9:
                Dn[n][m | (1 << n)] += D[n + 1][m] % 1000000000
            if n > 0:
                Dn[n][m | (1 << n)] += D[n - 1][m] % 1000000000
    D = Dn
print(sum([D[i][1023] for i in range(10)]) % 1000000000)
