import heapq

INF = 10 ** 9 + 1


def solve(N, A, Ops):
    pass


if __name__ == '__main__':
    N = int(input())
    A = list(map(int, input().split()))
    M = int(input())
    Ops: list[list[tuple]] = [[] for _ in range(N)]
    for _ in range(M):
        l, r, c = map(int, input().split())
        Ops[l].append((r, c))
        Ops[r].append((l, c))


"""
문자가 배열된 상태 S_k를 정의할 수 있음.
이에 따라 S_0에서 S_k로 가는 distance를 구할 수 있고, 이로 dijkstra를 전개하면 됨.
"""