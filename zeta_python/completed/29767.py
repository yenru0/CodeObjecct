class ToMaximize:
    def __init__(self, N: int, K: int, A: list[int]):
        self.N, self.K, self.A = N, K, A

    def solve(self):
        cumA = []
        ss = 0
        for i in range(self.N):
            ss += self.A[i]
            cumA.append((ss, i))
        cumA.sort(key=lambda x: x[0])

        popped = [cumA.pop()[0] for _ in range(self.K)]
        return sum(popped)


if __name__ == "__main__":
    N, K = map(int, input().split())
    A = list(map(int, input().split()))
    print(ToMaximize(N, K, A).solve())
