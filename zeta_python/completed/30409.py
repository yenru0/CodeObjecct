import sys

input = sys.stdin.readline


class WirePole:
    def __init__(self, N, H):
        self.N: int = N
        self.H: list[int] = H
        self.D_L = [0] * self.N
        self.D_R = [-10000000] * self.N

        # generate scheme
        s = []
        for i, v in enumerate(self.H):
            if s:
                flag = False
                bi, bv = s[-1]
                while bv < v:
                    s.pop()
                    if not s:
                        flag = True
                        break
                    bi, bv = s[-1]
                if flag:
                    self.D_L[i] = 0
                else:
                    self.D_L[i] = (bi - i) ** 2 + (bv - v) ** 2 + self.D_L[bi]
            else:
                self.D_L[i] = 0
            s.append((i, v))

        s = []
        for i, v in reversed(list(enumerate(self.H))):
            if s:
                bi, bv = s[-1]
                flag = False
                while bv < v:
                    s.pop()
                    if not s:
                        flag = True
                        break
                    bi, bv = s[-1]
                if flag:
                    self.D_R[i] = 0
                else:
                    self.D_R[i] = (bi - i) ** 2 + (bv - v) ** 2 + self.D_R[bi]
            else:
                self.D_R[i] = 0
            s.append((i, v))

    def solve(self, p: int) -> int:
        """
        S(p) = L(p) + R(p)
        """
        return self.D_L[p] + self.D_R[p]


if __name__ == "__main__":
    N = int(input())
    H = list(map(int, input().split()))
    Q = int(input())
    solver = WirePole(N, H)
    for _ in range(Q):
        print(solver.solve(int(input()) - 1))
