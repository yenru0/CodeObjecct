import sys

input = sys.stdin.readline

if __name__ == "__main__":
    S = [0]
    N = int(input())
    D = list(map(int, input().split()))
    Q = int(input())
    for i in range(N - 1):
        if D[i] > D[i + 1]:
            S.append(S[-1] + 1)
        else:
            S.append(S[-1])
    S.append(S[-1])
    for _ in range(Q):
        x, y = map(int, input().split())
        if x == y:
            print(0)
            continue
        print(S[y - 1] - S[x - 1])
