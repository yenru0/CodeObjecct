N, K = map(int, input().split())
Ws = [tuple(map(int, input().split())) for i in range(N)]  # weight, value

D = [[0] * (K + 1) for _ in range(N + 1)]


def solve(k):
    global D
    for n in range(1, N + 1):
        for k in range(1, K + 1):
            if Ws[n - 1][0] <= k:
                D[n][k] = max(Ws[n - 1][1] + D[n - 1][k - Ws[n - 1][0]], D[n - 1][k])
            else:
                D[n][k] = D[n - 1][k]
    return D[N][K]


print(solve(K))
