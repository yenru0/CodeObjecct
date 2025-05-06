TC = int(input())

for _ in range(TC):
    S = 0
    T = {}
    N = int(input())
    for _ in range(N):
        p, q = input().split()
        if q in T:
            T[q] += 1
        else:
            T[q] = 1
    for i in range()
