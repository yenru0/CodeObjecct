import sys
import math

input = sys.stdin.readline


def cost(time):
    if time > 100:
        return 10 + 3 * math.ceil((time - 100) / 50)
    else:
        return 10


if __name__ == "__main__":
    N = int(input())
    table: dict = {}
    for _ in range(N):
        time_str, name = input().rstrip().split()
        if name not in table:
            table[name] = (lambda a, b: a * 60 + b)(*map(int, time_str.split(":")))
        else:
            table[name] += (lambda a, b: a * 60 + b)(*map(int, time_str.split(":")))

    for c, n in sorted(
        [(cost(value), name) for name, value in table.items()],
        key=lambda x: (-x[0], x[1]),
    ):
        print(n, c)
