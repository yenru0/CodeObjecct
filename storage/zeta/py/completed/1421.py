import sys

input = sys.stdin.readline
relu = lambda x: x if x >= 0 else 0

if __name__ == "__main__":
    N, C, W = map(int, input().split())
    A = [int(input()) for _ in range(N)]
    costs = [
        sum(
            [
                relu(-((a // i - 1) * C if a % i == 0 else a // i * C) + a // i * W * i)
                for a in A
            ]
        )
        for i in range(1, max(A) + 1)
    ]

    print(max(costs))
