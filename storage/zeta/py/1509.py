import sys

input = sys.stdin.readline

class PalindromePartition:
    def __init__(self, s: str):
        self.s = s
        self.N = len(s)


if __name__ == "__main__":
    s = input().rstrip()
