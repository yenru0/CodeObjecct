class AdrenalineRushSolver:
    def __init__(self, n: int, final_order: list[int]):
        self.n = n
        self.final_order = final_order

    def solve(self) -> list[tuple[int, int]]:
        overtaken_history = []
        for i in range(self.n):
            ro = self.final_order.index(i)
            for j in range(ro + 1, self.n):
                overtaken_history.append((self.final_order[j], i))
            for j in range(self.n - 1, ro, -1):
                overtaken_history.append((i, self.final_order[j]))
            for j in range(ro, i, -1):
                overtaken_history.append((i, self.final_order[j - 1]))
                self.final_order[j], self.final_order[j - 1] = self.final_order[j - 1], self.final_order[j]

        return overtaken_history[::-1]


if __name__ == '__main__':
    n = int(input())
    final_order = list(map(lambda x: int(x) - 1, input().split()))

    solved = AdrenalineRushSolver(n, final_order).solve()
    print(len(solved))
    for b, a in solved:
        print(a + 1, b + 1)
