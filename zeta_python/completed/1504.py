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


def solve(N, E, v1, v2) -> int:
    heap = []
    distance_from_v1 = dijkstra(N, E, v1, )
    distance_from_v2 = dijkstra(N, E, v2, )
    ret = min(
        (sum((distance_from_v1[1], distance_from_v1[v2], distance_from_v2[N])),
         sum((distance_from_v2[1], distance_from_v2[v1], distance_from_v1[N])))
    )
    return ret if ret < 10 ** 9 + 1 else -1


if __name__ == '__main__':
    N, e = map(int, input().split())
    E = {}
    for i in range(1, N + 1):
        E[i] = []
    for _ in range(e):
        a, b, c = map(int, input().split())
        E[a].append((b, c))
        E[b].append((a, c))
    v1, v2 = map(int, input().split())
    print(solve(N, E, v1, v2))
