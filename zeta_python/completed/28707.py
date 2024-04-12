import sys
import heapq

input = sys.stdin.readline
INF = 10 ** 9 + 1

sorted_one = None


def is_sorted(arr):
    return arr == sorted(arr)


def swap(a, l, r):
    arr = a.copy()
    arr[l - 1], arr[r - 1] = arr[r - 1], arr[l - 1]
    return arr


def dijkstra(operators, start: list) -> dict:
    heap = []
    distance = {tuple(start): 0}
    heapq.heappush(heap, (0, start,))
    while heap:
        cost, now, = heapq.heappop(heap)
        if is_sorted(now):
            return distance

        if distance[tuple(now)] < cost:
            continue

        for l, r, weight in operators:
            new = swap(now, l, r)
            new_cost = cost + weight
            if tuple(new) not in distance:
                distance[tuple(new)] = INF
            if new_cost < distance[tuple(new)]:
                distance[tuple(new)] = new_cost
                heapq.heappush(heap, (new_cost, new))
    distance[tuple(sorted_one)] = -1
    return distance


def solve(N, A, Ops):
    distance = dijkstra(Ops, A)
    return distance[tuple(sorted_one)]


if __name__ == '__main__':
    N = int(input())
    A = list(map(int, input().split()))
    sorted_one = sorted(A)
    M = int(input())
    Ops: list = []
    for _ in range(M):
        l, r, c = map(int, input().split())
        Ops.append((l, r, c))
    print(solve(N, A, Ops))
"""
문자가 배열된 상태 S_k를 정의할 수 있음.
이에 따라 S_0에서 S_k로 가는 distance를 구할 수 있고, 이로 dijkstra를 전개하면 됨.
"""
