import sys

input = sys.stdin.readline


class TopoSort:
    def __init__(self, N, E):
        self.__N = N
        self.__edges = E
        self.__targets = [[] for _ in range(self.__N)]
        self.__indeg = [0] * self.__N
        for u, v in self.__edges:
            self.__indeg[v] += 1
            self.__targets[u].append(v)

    def sort(self) -> list[int]:
        indeg = self.__indeg.copy()
        Q = []
        for i, d in enumerate(indeg):
            if d == 0:
                Q.append(i)
        line = []
        while Q:
            u = Q.pop()
            line.append(u)
            for v in self.__targets[u]:
                indeg[v] -= 1
                if indeg[v] == 0:
                    Q.append(v)

        return line


if __name__ == "__main__":
    N, M = map(int, input().split())
    E = [tuple(map(lambda x: int(x) - 1, input().split())) for _ in range(M)]
    solver = TopoSort(N, E)
    print(" ".join(map(lambda x: str(int(x) + 1), solver.sort())))
