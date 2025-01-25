from collections import deque

MOVEABLE = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (0, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
]


class Board:
    def __init__(self, N, M, arr):
        self.size = (N, M)
        self.__arr = arr

    def check_wall(self, i, j):
        return (
            not (0 <= i < self.size[0] and 0 <= j < self.size[1])
            or self.__arr[self.size[1] * i + j]
        )


class AgentSwitching:
    def __init__(self, N, M, raw):
        self.agents_init = [(-1, -1), (-1, -1)]
        arr = []
        for i, s in enumerate(raw):
            for j, c in enumerate(s):
                if c == "X":
                    arr.append(1)
                elif c == "A":
                    self.agents_init[0] = (i, j)
                    arr.append(0)
                elif c == "B":
                    self.agents_init[1] = (i, j)
                    arr.append(0)
                else:
                    arr.append(0)
        self.board = Board(N, M, arr)

    def solve(self):
        vis = [
            [[[0 for _ in range(M)] for _ in range(N)] for _ in range(M)]
            for _ in range(N)
        ]
        D = deque([(self.agents_init[0], self.agents_init[1], 0)])
        while D:
            (ai, aj), (bi, bj), turn = D.popleft()

            if vis[ai][aj][bi][bj]:
                continue
            if (ai, aj) == self.agents_init[1] and (bi, bj) == self.agents_init[0]:

                return turn
            a_possible = [
                (ai + mi, aj + mj)
                for mi, mj in MOVEABLE
                if not self.board.check_wall(ai + mi, aj + mj)
            ]
            b_possible = [
                (bi + mi, bj + mj)
                for mi, mj in MOVEABLE
                if not self.board.check_wall(bi + mi, bj + mj)
            ]
            for new_a in a_possible:
                for new_b in b_possible:
                    if new_a == new_b:
                        continue
                    if new_a == (bi, bj) and new_b == (ai, aj):
                        continue

                    D.append((new_a, new_b, turn + 1))
            vis[ai][aj][bi][bj] = 1

        return -1


if __name__ == "__main__":
    N, M = map(int, input().split())
    raw_consume = [input().rstrip() for _ in range(N)]
    solver = AgentSwitching(N, M, raw_consume)
    print(solver.solve())
