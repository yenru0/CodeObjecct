N = int(input())
P = []
last = 1
while N != 0:
    if 2*N > last:
        if last == 1:
            last = 2
        for i in range(last, 2*N+1):
            for p in P:
                if i % p == 0:
                    break
                elif p ** 2 > i:
                    P.append(i)
                    break
            else:
                P.append(i)
    c = 0
    for p in P:
        if N < p <= 2 * N:
            c += 1
        elif p > 2*N:
            break
    if last < 2*N:
        last = 2*N

    print(c)

    N = int(input())
