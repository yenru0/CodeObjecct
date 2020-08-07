import sys

P = [2]
K = [False] * 1000005
K[2] = True
for i in range(2, 1000000 - 1):
    for p in P:
        if i % p == 0:
            break
        elif p * p > i:
            P.append(i)
            K[i] = True
            break
    else:
        P.append(i)
        K[i] = True
        break
while True:
    n = int(sys.stdin.readline())
    if n == 0:
        break
    i = 2
    while not (K[i] and K[n - i]):
        i += 1
    else:
        print(n, "=", i, "+", n - i)
