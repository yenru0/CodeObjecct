import sys

input = sys.stdin.readline


class Palindrome:
    def __init__(self, N: int, arr: list[int]):
        self.N = N
        self.arr = arr
        self.P = [[0 for _ in range(N)] for _ in range(N)]

        for i in range(self.N):
            s = i
            e = i
            while 0 <= s and e < self.N:
                if self.arr[s] == self.arr[e]:
                    self.P[s][e] = 1
                else:
                    break
                s -= 1
                e += 1

        for i in range(self.N - 1):
            s = i
            e = i + 1
            while 0 <= s and e < self.N:
                if self.arr[s] == self.arr[e]:
                    self.P[s][e] = 1
                else:
                    break
                s -= 1
                e += 1

    def is_partial_valid(self, s: int, e: int) -> int:
        return self.P[s][e]


if __name__ == "__main__":
    N = int(input())
    arr = list(map(int, input().split()))
    solver = Palindrome(N, arr)
    M = int(input())
    for s, e in [tuple(map(lambda x: int(x) - 1, input().split())) for _ in range(M)]:
        print(solver.is_partial_valid(s, e))
