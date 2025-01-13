import sys

input = sys.stdin.readline

INF = 1 << 30


def bellman_ford(N, E, start=1):
    dist = [INF] * (N + 1)
    parent = [0] * (N + 1)

    dist[start] = 0
    for _ in range(1, N + 1):
        for u, v, w in E:
            if dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
                parent[v] = u

    for u, v, w in E:
        if dist[u] + w < dist[v]:
            return -INF
    return dist[N]


if __name__ == "__main__":
    T = int(input())
    for _ in range(T):
        N, M, W = map(int, input().split())
        E = []
        for s, e, w in [map(int, input().split()) for _ in range(M)]:
            E.append((s, e, w))
            E.append((e, s, w))
        for s, e, w in [map(int, input().split()) for _ in range(W)]:
            E.append((s, e, -w))
        print("YES" if bellman_ford(N, E) < 0 else "NO")
