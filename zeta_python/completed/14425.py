import sys

input = sys.stdin.readline

if __name__ == "__main__":
    N, M = map(int, input().split())
    S = {input().rstrip() for _ in range(N)}
    print(sum(1 for _ in range(M) if input().rstrip() in S))
