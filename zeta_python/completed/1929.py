M, N = map(int, input().split())
P = []

for i in range(2, N+1):
    for p in P:
        if i % p == 0:
            break
        elif p ** 2 > i:
            if i >= M:
                print(i)
            P.append(i)
            break
    else:
        if i >= M:
            print(i)
        P.append(i)
