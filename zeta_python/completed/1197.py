import sys

input = sys.stdin.readline


class MinimumSpanningTree:
    def __init__(self, V: int, E: int, edges: list[tuple[int]]):
        self.V = V
        self.E = E
        self.edges: list[tuple[int]] = edges

        self.edges.sort(key=lambda x: x[2])
        self.__parents = [i for i in range(self.V + 1)]

    def __root(self, x: int) -> int:
        node = x
        while node != self.__parents[node]:
            node = self.__parents[node]
        return node

    def __union(self, x: int, y: int):
        rx = self.__root(x)
        ry = self.__root(y)
        if rx > ry:
            self.__parents[ry] = rx
        else:
            self.__parents[rx] = ry

    def solve(self):
        mst = []
        for e in self.edges:
            a = e[0]
            b = e[1]
            if self.__root(a) == self.__root(b):
                continue
            mst.append(e)
            self.__union(a, b)

            if len(mst) == self.V - 1:
                break
        return sum(map(lambda x: x[2], mst))


if __name__ == "__main__":
    V, E = map(int, input().split())
    edges = [tuple(map(int, input().split())) for _ in range(E)]
    solver = MinimumSpanningTree(V, E, edges)
    print(solver.solve())
