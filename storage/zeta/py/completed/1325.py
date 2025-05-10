from collections import deque
import sys

input = sys.stdin.readline


def bfs(N, H, start):
    Vsx = [0] * (N + 1)
    count = 0
    D = deque()
    D.append(start)
    Vsx[start] = 1

    while D:
        now = D.pop()
        count += 1
        for target in H[now]:
            if not Vsx[target]:
                D.append(target)
                Vsx[target] = 1

    return count


def solve(N, M, H):
    S = [bfs(N, H, i) for i in range(1, N + 1)]
    m = max(S)
    return [i + 1 for i, v in enumerate(S) if v == m]


if __name__ == "__main__":
    N, M = map(int, input().split())
    H = [[] for _ in range(N + 1)]
    for _ in range(M):
        s, e = map(int, input().split())
        H[e].append(s)
    print(*solve(N, M, H))
