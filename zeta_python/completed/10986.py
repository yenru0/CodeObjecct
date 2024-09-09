import sys
import math

input = sys.stdin.readline

if __name__ == "__main__":
    N, M = map(int, input().split())
    A = list(map(int, input().split()))
    S = [(0, 0)]
    C = [0] * 1001
    C[0] = 1
    for i in range(N):
        s = ((S[-1][0] + A[i]) % M, i + 1)
        S.append(s)
        C[s[0]] += 1

    print(sum([math.comb(c, 2) for c in C]))
