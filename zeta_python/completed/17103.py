import sys

T = int(sys.stdin.readline())

P = [2]
K = [0] * 1000005
K[2] = True
for i in range(2, 1000000 - 1):
    for p in P:
        if i % p == 0:
            break
        elif p * p > i:
            P.append(i)
            K[i] = 1
            break
    else:
        P.append(i)
        K[i] = 1
        break
for _ in range(T):
    n = int(sys.stdin.readline())
    c = 0
    for i in range(2, n//2+1):
        if K[i] and K[n - i]:
            c += 1
        i += 1
    print(c)
