N = int(input())

I = [list(map(int, input().split())) for i in range(N)]
T = [[0 for _ in range(i + 1)] for i in range(N)]
T[0][0] = I[0][0]

for i in range(1, N):
    for j in range(i + 1):
        T[i][j] = max([T[i - 1][j - 1] if j > 0 else 0, T[i - 1][j] if i > j else 0]) + I[i][j]

print(max(T[-1]))
