import sys

input = sys.stdin.readline


class DiffGameSolver:
    def __init__(self, N, A: list[int], B: list[int], betas: list[int]):
        self.__diff_max_by_columns = []
        for i in range(N):
            self.__diff_max_by_columns.append(
                max(abs(A[j][i] - B[j][i]) for j in range(N))
            )

        self.__betas = betas

    def solve(self) -> int:
        return sum([self.__diff_max_by_columns[beta] for beta in self.__betas])


if __name__ == "__main__":
    N, M = map(int, input().split())
    A = [list(map(int, input().split())) for _ in range(N)]
    B = [list(map(int, input().split())) for _ in range(N)]
    betas = list(map(lambda x: int(x) - 1, input().split()))

    solver = DiffGameSolver(N, A, B, betas)
    print(solver.solve())
