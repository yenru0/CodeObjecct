M, N = int(input()), int(input())
P = []
MP = []


for i in range(2, N+1):
    for p in P:
        if i % p == 0:
            break
        elif p ** 2 > i:
            if i >= M:
                MP.append(i)
            P.append(i)
            break
    else:
        if i >= M:
            MP.append(i)
        P.append(i)

if MP:
    print(sum(MP))
    print(MP[0])
else:
    print(-1)
