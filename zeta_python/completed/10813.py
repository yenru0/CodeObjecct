N, M = map(int, input().split())
L = [i for i in range(1, N + 1)]


def swap(i, j):
    global L
    L[i - 1], L[j - 1] = L[j - 1], L[i - 1]


for _ in range(M):
    i, j = map(int, input().split())
    swap(i, j)

print(" ".join(map(str, L)))
