import sys
import bisect

input = sys.stdin.readline

Mem = [1, 2, 3]


def fib_count(a: int, b: int):
    if b <= Mem[-1]:
        pass
    else:
        while b > Mem[-1]:
            Mem.append(Mem[-1] + Mem[-2])

    right = bisect.bisect_right(Mem, b)
    left = bisect.bisect_left(Mem, a)

    return right - left


if __name__ == "__main__":
    while True:
        a, b = map(int, input().split())
        if a == b == 0:
            break
        else:
            print(fib_count(a, b))
