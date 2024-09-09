import sys

input = sys.stdin.readline

if __name__ == "__main__":
    N, M = map(int, input().split())
    A = list(map(int, input().split()))
    S = [0]
    for i in range(N):
        S.append(S[-1] + A[i])
    for _ in range(M):
        i, j = map(int, input().split())
        print(S[j] - S[i - 1])
