class PizzaOven:
    def __init__(self, D, N, r_oven, r_pizzas):
        self.D: int = D
        self.N: int = N
        self.r1: list[int] = r_oven
        self.r2: list[int] = r_pizzas

    def solve(self):
        pizzas = self.r2.copy()[::-1]
        able_radius_map = []
        able_radius_map.append(self.r1[0])
        for i in range(1, self.D):
            if able_radius_map[-1] < self.r1[i]:
                able_radius_map.append(able_radius_map[-1])
            else:
                able_radius_map.append(self.r1[i])

        depth = self.D
        while pizzas:
            depth -= 1
            r = pizzas.pop()
            while able_radius_map[depth] < r:
                if depth == -1:
                    break
                depth -= 1
            if depth == -1:
                break

        return depth + 1


if __name__ == "__main__":
    D, N = map(int, input().split())
    r1 = list(map(int, input().split()))
    r2 = list(map(int, input().split()))
    solver = PizzaOven(D, N, r1, r2)
    print(solver.solve())
