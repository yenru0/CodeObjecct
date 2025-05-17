import sys

input = sys.stdin.readline


class SeongjunEncryption:
    def __init__(self, n, t):
        self.n = n
        self.t = t

    def solve(self) -> bool:
        return self.t[0] == 'A' and self.t[-1] == 'B'


if __name__ == "__main__":
    n = int(input())
    t = input().strip()

    print("Yes" if SeongjunEncryption(n, t).solve() else "No")
