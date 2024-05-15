import sys
import heapq

input = sys.stdin.readline


def solve(N, K, X, Y, E_X, E_Y):
    G_X = [[] for _ in range(N + 1)]
    G_Y = [[] for _ in range(N + 1)]
    for s, e, d in E_X:
        G_X[s].append((e, d))
        G_X[e].append((s, d))

    for s, e, d in E_Y:
        G_Y[s].append((e, d))
        G_Y[e].append((s, d))

    dist = [float("inf") for _ in range(N + 1)]
    dist[1] = 0

    pq = []
    heapq.heappush(pq, (dist[1], 1))

    while pq:
        now_dist, now_pos = heapq.heappop(pq)
        if dist[now_pos] < now_dist:
            continue

        for target, cost in G_X[now_pos]:
            new_cost = now_dist + cost
            if new_cost < dist[target]:
                dist[target] = new_cost
                heapq.heappush(pq, (new_cost, target))

        for target, cost in G_Y[now_pos]:
            if now_dist < K:
                new_cost = K + cost
            else:
                new_cost = now_dist + cost

            if new_cost < dist[target]:
                dist[target] = new_cost
                heapq.heappush(pq, (new_cost, target))
    return dist[N]


if __name__ == "__main__":
    N, K, X, Y = map(int, input().split())
    E_X = list(map(int, input().split()) for _ in range(X))
    E_Y = list(map(int, input().split()) for _ in range(Y))
    print(solve(N, K, X, Y, E_X, E_Y))
