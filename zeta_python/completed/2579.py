N = int(input())
I = [int(input()) for _ in range(N)]
T = [[0 for j in range(3)] for i in range(N)]  # T[n][state]
T[0][0] = 0
T[0][1] = I[0]
T[0][2] = I[0]

for i in range(1, N):
    T[i][2] = T[i - 1][1] + I[i]
    T[i][1] = T[i - 1][0] + I[i]
    T[i][0] = max([T[i - 1][1], T[i - 1][2]])

print(max(T[N-1][1:]))
