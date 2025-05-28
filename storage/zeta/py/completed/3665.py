from collections import deque

import sys

input = sys.stdin.readline


class FinalRankSolver:
    def __init__(self, n: int, last_rank: list[int], m: int, changes: list[list[int]]):
        self.n = n
        self.last_rank = last_rank
        self.m = m
        self.changes = changes

    def solve(self) -> int | list[int]:

        flag = 1

        adj = [[0 for _ in range(self.n)] for _ in range(self.n)]

        indeg = [0] * self.n

        last_rank_index = [0] * self.n

        for i, r in enumerate(self.last_rank):
            last_rank_index[r - 1] = i

        for i in range(1, self.n):
            for j in range(i):
                adj[self.last_rank[i] - 1][self.last_rank[j] - 1] = 1
                indeg[self.last_rank[j] - 1] += 1

        for first, second in self.changes:
            if last_rank_index[first - 1] > last_rank_index[second - 1]:
                adj[first - 1][second - 1] = 0
                adj[second - 1][first - 1] = 1
                indeg[second - 1] -= 1
                indeg[first - 1] += 1
            else:
                first, second = second, first
                adj[first - 1][second - 1] = 0
                adj[second - 1][first - 1] = 1
                indeg[second - 1] -= 1
                indeg[first - 1] += 1

        new_rank = []
        q = deque()
        for i, v in enumerate(indeg):
            if v == 0:
                q.append(i)

        while q:
            index = q.popleft()
            indeg[index] = -1
            new_rank.append(index + 1)
            for ix, vi in enumerate(adj[index]):
                if vi == 1:
                    indeg[ix] -= 1
            for i, v in enumerate(indeg):
                if v == 0:
                    q.append(i)

        if any(deg != -1 for deg in indeg):
            flag = 0

        if flag:
            return new_rank
        else:
            return flag


if __name__ == "__main__":
    tc = int(input())

    for _ in range(tc):
        n = int(input())
        last_rank = list(map(int, input().split()))
        m = int(input())
        changes = [list(map(int, input().split())) for i in range(m)]

        res = FinalRankSolver(n, last_rank, m, changes).solve()
        if res == -1:
            print("?")
        elif res == 0:
            print("IMPOSSIBLE")
        else:
            print(" ".join(map(str, res[::-1])))
