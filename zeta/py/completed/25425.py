class RankRange:
    def __init__(self, N, M, a, K):
        # N: team, M: max person per team, a: remains, K: my team
        self.N, self.M, self.a, self.K = N, M, a, K

    def solve(self):
        worst, best = -1, -1

        except_us = self.a - self.K
        # worst case 1
        if except_us >= self.N - 1:
            worst = self.N
        else:
            worst = except_us + 1

        # best case
        d, m = divmod(except_us, self.M)
        best = d + 1 if m == 0 else d + 2
        return worst, best


if __name__ == "__main__":
    N, M, a, K = map(int, input().split())
    print(*RankRange(N, M, a, K).solve())
