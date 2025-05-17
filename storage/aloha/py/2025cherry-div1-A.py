import sys

input = sys.stdin.readline


class FibonacciCoin:
    def __init__(self, n, arr):
        self.n = n
        self.arr = arr

    def solve(self) -> list[int]:
        cnts: list[int] = []
        curr = [0] * 1_200_000
        if self.arr[0] == 1 or self.arr[0] == 2:
            curr[2] = 1
        else:
            curr[self.arr[0]] = 1
        cnts.append(1)
        for i in range(1, self.n):
            cnt = cnts[-1]
            if self.arr[i] == 1 or self.arr[i] == 2:
                curr[2] += 1
                if curr[2] == 2:
                    curr[2] = 0
                    curr[3] += 1
                else:
                    cnt += 1
                t = 3
            else:
                curr[self.arr[i]] += 1
                t = self.arr[i]
                cnt += 1
            while True:
                if curr[t] >= 1 and curr[t - 1] >= 1:
                    curr[t] -= 1
                    curr[t - 1] -= 1
                    curr[t + 1] += 1
                    cnt -= 1
                    t = t + 1
                elif curr[t] >= 1 and curr[t + 1] >= 1:
                    s = min([curr[t], curr[t + 1]])
                    curr[t] -= 1
                    curr[t + 1] -= 1
                    curr[t + 2] += 1
                    cnt -= 1
                    t = t + 2
                else:
                    break
            cnts.append(cnt)
        return cnts


if __name__ == '__main__':
    n = int(input())
    arr = list(map(int, input().split()))

    solver = FibonacciCoin(n, arr)

    print(*solver.solve(), sep=' ')
