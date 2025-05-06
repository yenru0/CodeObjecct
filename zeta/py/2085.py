import sys

input = sys.stdin.readline


def count_nums(post: list[str], base: int):
    n = len(post)
    subpost = [[0 for _ in range(n)] for _ in range(n)]
    cnts = [[0 for _ in range(n)] for _ in range(n)]
    P = [0] * n
    for i in range(n):
        cnt = 0
        for j in range(i, n):
            subpost[i][j] = int("".join(post[i:j + 1]))
    for i in range(n):
        cnt = 0
        for j in range(i+1):
            if subpost[j][i] < base:
                pass

class BaseSpliter:
    def __init__(self, s: str):
        self.s: str = s
        self.post = []
        if len(self.s) <= 1:
            return
        self.flag = False
        if self.s[0] == "0":
            if len(self.s) > 1 and self.s[1] == 0:
                pass
            else:
                self.flag = True
            return
        for c in self.s:
            if c == "0":
                self.post[-1] += c
            else:
                self.post.append(c)
        print(self.post)

    def solve(self) -> int:
        if not self.post:
            return 0
        elif self.flag:
            return 1


if __name__ == "__main__":
    S = input().rstrip()
    solver = BaseSpliter(S)
    # print(solver.solve())
    count_nums(["1", "2", "3"], 4)
