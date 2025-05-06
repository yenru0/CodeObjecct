import sys

input = sys.stdin.readline


class TractorPath:
    def __init__(self, N: int, arr: list[list[int]]):
        self.N = N
        self.arr = arr
        self.vis = [[0 for _ in range(N)] for _ in range(N)]

    def isValid(self) -> bool:
        D = [(0, 0)]
        while D:
            x, y = D.pop()
            if not self.vis[x][y]:
                self.vis[x][y] = 1
            else:
                continue
            if x == self.N - 1 and y == self.N - 1:
                return True
            if x != self.N - 1 and not self.arr[x + 1][y]:
                D.append((x + 1, y))
            if y != self.N - 1 and not self.arr[x][y + 1]:
                D.append((x, y + 1))
        return False

    def solve(self) -> str:
        return "Yes" if self.isValid() else "No"


if __name__ == "__main__":
    N = int(input())
    arr = [
        list(map(lambda s: 0 if s == "." else 1, input().rstrip())) for _ in range(N)
    ]
    solver = TractorPath(N, arr)
    print(solver.solve())
