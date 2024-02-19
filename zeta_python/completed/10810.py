N, M = map(int, input().split())
L = [0 for _ in range(N)]
for _ in range(M):
    i, j, k = map(int, input().split())
    for m in range(i - 1, j):
        L[m] = k

print(" ".join(map(str, L)))
