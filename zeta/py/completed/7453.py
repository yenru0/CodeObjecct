import sys

input = sys.stdin.readline


class FourArray:
    def __init__(self, N, mat):
        self.__N = N
        self.__arr: list[list[int]] = mat  # shape = (4, N)

        self.__firsts = dict()
        for i in range(self.__N):
            for j in range(self.__N):
                first = self.__arr[i][0] + self.__arr[j][1]
                if first in self.__firsts:
                    self.__firsts[first] += 1
                else:
                    self.__firsts[first] = 1

    def solve(self) -> int:
        s = 0
        for i in range(self.__N):
            for j in range(self.__N):
                target = -(self.__arr[i][2] + self.__arr[j][3])
                if target in self.__firsts:
                    s += self.__firsts[target]

        return s


if __name__ == "__main__":
    N = int(input())
    solver = FourArray(N, [list(map(int, input().split())) for _ in range(N)])
    print(solver.solve())
