import heapq
import sys

input = sys.stdin.readline


def dijkstra(N, E, start) -> list[int]:
    heap = []
    distance = [10 ** 9 + 1 for _ in range(N + 1)]
    distance[start] = 0
    heapq.heappush(heap, (0, start,))
    while heap:
        cost, now, = heapq.heappop(heap)
        if distance[now] < cost:
            continue

        for new, weight in E[now]:
            new_cost = cost + weight
            if new_cost < distance[new]:
                distance[new] = new_cost
                heapq.heappush(heap, (new_cost, new))

    return distance


def solve(N, M, E, iE, X):
    come = dijkstra(N, E, X)
    gone = dijkstra(N, iE, X)
    return max([come[i] + gone[i] for i in range(1, N + 1)])


if __name__ == '__main__':
    N, M, X = map(int, input().split())
    E = {i: [] for i in range(1, N + 1)}
    iE = {i: [] for i in range(1, N + 1)}
    for _ in range(M):
        A, B, T = map(int, input().split())
        E[A].append((B, T))
        iE[B].append((A, T))
    print(solve(N, M, E, iE, X))
