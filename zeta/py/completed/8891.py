from dataclasses import dataclass


@dataclass
class Point:
    x: int
    y: int

    def __add__(self, p: "Point"):
        return Point(self.x + p.x, self.y + p.y)


S = lambda n: n * (n + 1) // 2


def point_to_num(point: Point) -> int:
    return S(point.x + point.y - 2) + point.x


def num_to_point(num: int) -> Point:
    n = 0
    temp1 = S(n)
    temp2 = S(n + 1)
    while not (temp1 < num <= temp2):
        n += 1
        temp1, temp2 = temp2, S(n + 1)
    return Point(num - temp1, n + 2 - (num - temp1))


if __name__ == "__main__":
    N = int(input())
    # fmt: off
    print(*[point_to_num(sum(map(num_to_point, map(int, input().split())), start=Point(0, 0))) for _ in range(N)], sep='\n')            
