import sys

input = sys.stdin.readline


class SC2ReasonableRanking:
    def __init__(self, n: int, arr: list[list[int]]):
        self.n: int = n
        self.arr: list[list[int]] = arr
        self.ranks: list[int] = []

    def insert(self, idx: int):
        if self.ranks:
            if self.arr[self.ranks[-1]][idx]:
                self.ranks.append(idx)
            else:
                for i in range(len(self.ranks) - 2, -1, -1):
                    if (
                        self.arr[self.ranks[i]][idx]
                        and self.arr[idx][self.ranks[i + 1]]
                    ):
                        self.ranks.insert(i + 1, idx)
                        break
                else:
                    self.ranks.insert(0, idx)
        else:
            self.ranks.append(idx)

    def solve(self) -> str:
        if not self.ranks:
            for i in range(self.n):
                self.insert(i)
        return self.ranks


if __name__ == "__main__":
    while (n := int(input())) != 0:
        arr = [list(map(int, input().strip())) for _ in range(n)]
        solver = SC2ReasonableRanking(n, arr)
        print(*map(lambda x: x + 1, solver.solve()), end="")
        print()
