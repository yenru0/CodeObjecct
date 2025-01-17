import sys

input = sys.stdin.readline


class HappyCow:
    def __init__(self, N: int, H: list[int]):
        self.__N: int = N
        self.__happiness: list[int] = H
        self.__total: list[int] = [[0] * (self.__N + 2) for _ in range(self.__N + 2)]

        for i in range(1, self.__N + 1):
            day = i
            self.__total[i][self.__N] = (
                self.__total[i - 1][self.__N] + day * self.__happiness[i - 1]
            )
            self.__total[0][self.__N - i] = (
                self.__total[0][self.__N - i + 1] + day * self.__happiness[self.__N - i]
            )

        for i in range(1, self.__N):
            for j in range(self.__N - 1, i - 1, -1):
                self.__total[i][j] = max(
                    [
                        self.__total[i - 1][j]
                        + self.__happiness[i - 1] * (self.__N - j + i),
                        self.__total[i][j + 1]
                        + self.__happiness[j] * (self.__N - j + i),
                    ]
                )

    def solve(self) -> int:
        return max([self.__total[i][i] for i in range(self.__N)])


if __name__ == "__main__":
    N = int(input())
    H = list(map(int, input().split()))
    solver = HappyCow(N, H)
    print(solver.solve())
