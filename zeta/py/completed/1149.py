N = int(input())
C = [list(map(int, input().split())) for _ in range(N)]

T = [[0, 0, 0] for i in range(N)]
T[0] = C[0][:]

for i in range(1, N):
    T[i][0] = min((T[i - 1][1], T[i - 1][2])) + C[i][0]
    T[i][1] = min((T[i - 1][0], T[i - 1][2])) + C[i][1]
    T[i][2] = min((T[i - 1][0], T[i - 1][1])) + C[i][2]

print(min((T[N-1][0], T[N-1][1], T[N-1][2])))
