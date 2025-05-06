class PalindromeGenerator:
    def __init__(self, N, K):
        self.N, self.K = N, K

    def solve(self) -> str:
        S = "abcdefghijklmnopqrstuvwxyz"
        return "a" * (self.N - (self.K - 1)) + S[1 : self.K]


if __name__ == "__main__":
    # 3 1 => aaa
    # N 1 =. a...a.
    # 3 2 => aba
    # 4 2 => aaab
    # aaaaaaaaaaaaaaabc
    N, K = map(int, input().split())
    print(PalindromeGenerator(N, K).solve())
