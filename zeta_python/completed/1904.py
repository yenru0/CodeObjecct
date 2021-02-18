N = int(input())

M = [1, 2] + [0 for _ in range(1000000 - 1 - 1)]

for i in range(2, N):
    M[i] = (M[i - 1] + M[i - 2]) % 15746

print(M[N - 1])
