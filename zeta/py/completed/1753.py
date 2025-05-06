import sys
import heapq as hq

input = sys.stdin.readline


def solve(V, E, K) -> list[int]:
    W = [1 << 31 for _ in range(V + 1)]
    W[K] = 0
    D = []

    hq.heappush(D, (W[K], K))

    while D:
        d, k = hq.heappop(D)

        if d > W[k]:
            continue

        for v, w in E[k]:
            nd = d + w
            if W[v] > nd:
                W[v] = nd
                hq.heappush(D, (nd, v))

    return W


def pformat(x):
    if x >= 1 << 31:
        return 'INF'
    else:
        return x


if __name__ == '__main__':
    V, n = map(int, input().split())
    K = int(input())
    E = {}
    for i in range(1, V + 1):
        E[i] = []
    for _ in range(n):
        u, v, w = map(int, input().split())
        E[u].append((v, w))

    [print(pformat(i)) for i in solve(V, E, K)[1:]]
