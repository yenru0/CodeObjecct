T = int(input())
P = [2]
K = [False, False, True] + [False for i in range(9999)]
last = 2
for _ in range(T):
    n = int(input())
    if n - 2 > last:
        for i in range(last, n - 1):
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
    i = 0
    while not (K[n//2 - i] and K[n//2 + i]):
        i += 1
    if n - 1 > last:
        last = n - 1
    print(n//2 - i, n//2 + i)
# 골드바흐로 소수 여부를 False/True로 구별해 while 돌리는게 best
