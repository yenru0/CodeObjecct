N, r, c = map(int, input().split())

Z = lambda r, c: r * 2 + c

S = 0

for i in range(N - 1, -1, -1):
    S += (4 ** i) * Z(r // (2 ** i), c // (2 ** i))
    r, c = r % (2 ** i), c % (2 ** i)
print(S)
