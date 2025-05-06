import sys
sys.setrecursionlimit(12000)
N, K = map(int, input().split())
Mem = [[0]*1001for i in range(1001)]

def C(n, k):
    if n == 1:
        return 1
    elif k == 0 or k == n:
        return 1
    if Mem[n][k] != 0:
        return Mem[n][k]
    t = (C(n - 1, k) + C(n - 1, k - 1)) % 10007
    Mem[n][k] = t
    return t

print(C(N, K))
