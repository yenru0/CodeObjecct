N = int(input())
A = list(map(int, input().split()))

T = [0 for _ in range(N)]
T[0] = 1

for i in range(1, N):
    t = []
    for j in range(i):
        if A[j] < A[i]:
            t.append(T[j] + 1)
    else:
        t.append(1)
    T[i] = max(t)
print(max(T))
