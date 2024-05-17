import sys
import heapq

input = sys.stdin.readline


def solve(N, Cl):
    Cl.sort()

    pq = []
    heapq.heappush(pq, Cl[0][1])
    for i in range(1, N):
        if pq[0] <= Cl[i][0]:
            heapq.heappop(pq)
        heapq.heappush(pq, Cl[i][1])

    return len(pq)


if __name__ == "__main__":
    N = int(input())
    Cl = [tuple(map(int, input().split())) for _ in range(N)]
    print(solve(N, Cl))
