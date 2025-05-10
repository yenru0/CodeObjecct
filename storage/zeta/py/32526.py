class ConstructString:
    def __init__(self, n, k):
        self.n = n
        self.k = k

    def solve(self) -> tuple[str]:
        if self.k == 0:
            if self.n <= 2:
                return ("No",)
            else:
                return "Yes", "abc" * self.n // 3 + "abc"[: self.n % 3]
        elif self.k == self.n - 1:
            return "Yes", "a" * self.n

        s = [-1] * self.n
        for i in range(self.k):
            s[i] = 0
            s[self.n - i - 1] = 0
        
        if self.n - self.k - 1 >= self.k and s[self.n - self.k - 1] == -1:
            s[self.n - self.k - 1] = 1
        
        for i in range(self.n):
            if s[i] == -1:
                s[i] = 2

        return "Yes", s 
if __name__ == "__main__":
    n, k = map(int, input().split())
    solver = ConstructString(n, k)
    print(*solver.solve(), sep="\n")
