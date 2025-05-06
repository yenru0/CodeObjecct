import sys
from math import atan2, cos, sin, pi

input = sys.stdin.readline


def transpose(type: int, a: float, b: float) -> tuple[float]:
    if type == 1:
        m = atan2(b, a)
        return round((a**2 + b**2) ** (1 / 2), 7), round(m if m >= 0 else 2 * pi + m, 7)
    else:
        return round(a * cos(b), 7), round(a * sin(b), 7)


if __name__ == "__main__":
    T = int(input())
    # fmt: off
    [print(*transpose(int(input()), *map(float, input().split()))) for _ in range(T)]
