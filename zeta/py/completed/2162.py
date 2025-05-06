import sys
from collections import Counter

input = sys.stdin.readline


def ccw(p1, p2, p3):
    # fmt: off
    s = (p1[0] * p2[1] + p2[0] * p3[1] + p3[0] * p1[1]) \
        - (p1[1] * p2[0] + p2[1] * p3[0] + p3[1] * p1[0])
    # fmt: on
    if s > 0:
        return 1
    elif s == 0:
        return 0
    else:
        return -1


class LinesGroup:
    def __init__(self, N: int, lines: list):
        self.__lines = lines
        self.__parents = [i for i in range(N)]
        for i in range(N):
            for j in range(i + 1, N):
                if self.intersect(i, j):
                    self.merge(i, j)
        self.__roots = [self.root(i) for i in range(N)]
        self.__counters: Counter = Counter(self.__roots)

    def intersect(self, a: int, b: int) -> bool:
        a: tuple[int] = self.__lines[a]
        b: tuple[int] = self.__lines[b]
        p1, p2 = (a[0], a[1]), (a[2], a[3])
        p3, p4 = (b[0], b[1]), (b[2], b[3])
        p1p2 = ccw(p1, p2, p3) * ccw(p1, p2, p4)
        p3p4 = ccw(p3, p4, p1) * ccw(p3, p4, p2)
        if p1p2 == 0 and p3p4 == 0:
            if p1 > p2:
                p1, p2 = p2, p1
            if p3 > p4:
                p3, p4 = p4, p3
            return p3 <= p2 and p1 <= p4
        return p1p2 <= 0 and p3p4 <= 0

    def merge(self, a, b):
        rb, ra = self.root(b), self.root(a)
        if ra < rb:
            self.__parents[rb] = ra
        else:
            self.__parents[ra] = rb

    def root(self, e) -> int:
        node = e
        while node != self.__parents[node]:
            node = self.__parents[node]
        return node

    @property
    def num_groups(self):
        return len(self.__counters)

    @property
    def num_max(self):
        return self.__counters.most_common(1)[0][1]


if __name__ == "__main__":
    N = int(input())
    groups = LinesGroup(N, [tuple(map(int, input().split())) for _ in range(N)])
    print(groups.num_groups)
    print(groups.num_max)
