import heapq
import sys

input = sys.stdin.readline

INF = 10 ** 9 + 1


def dijkstra(N, V, E, start) -> list[int]:
    heap = []
    distance = [INF*INF for _ in range(N)]
    distance[start] = 0
    heapq.heappush(heap, (0, start,))
    while heap:
        cost, now, = heapq.heappop(heap)
        if distance[now] < cost:
            continue

        for new, weight in E[now]:
            new_cost = cost + weight
            if V[new] != V[now]:
                new_cost += INF
            if new_cost < distance[new]:
                distance[new] = new_cost
                heapq.heappush(heap, (new_cost, new))

    return distance


def solve(N, M, V, E):
    distance = dijkstra(N, V, E, 0)
    return distance[M] // INF, distance[M] % INF


if __name__ == '__main__':
    N, M = map(int, input().split())
    V = [int(input()) for _ in range(N)]
    E = [[] for i in range(N)]
    for i in range(N):
        for j, w in enumerate(map(int, input().split())):
            if w != 0:
                E[i].append((j, w))

    print(*solve(N, M, V, E))
