import math
from decimal import Decimal, getcontext

getcontext().prec = 32


class ДимаЗарплата:
    TRI = Decimal(1) / Decimal(3)

    def __init__(self, k: int):
        self.k = k

    def solve(self) -> str:
        u = 1
        minima = (Decimal(10) ** (u - 1)) ** self.TRI
        maxima = (Decimal(10) ** (u)) ** self.TRI
        delta = 2
        cnt = delta * u
        now = self.k
        while now - cnt > 0:
            now -= cnt
            u += 1

            if (u - 1) % 3 != 0:
                minima = (Decimal(10) ** (u - 1)) ** self.TRI
            else:
                minima = Decimal(10) ** ((u - 1) // 3)

            if u % 3 != 0:
                maxima = (Decimal(10) ** u) ** self.TRI
            else:
                maxima = Decimal(10) ** (u // 3)

            if u % 3 == 0:
                delta = int(maxima) - math.ceil(minima)
            elif (u - 1) % 3 == 0:
                delta = int(maxima) - int(minima) + 1
            else:
                delta = int(maxima) - math.ceil(minima) + 1
            cnt = u * delta

        if u == 1:
            return str((math.ceil(minima) + now - 1) ** 3)[0]
        else:
            d, m = divmod(now - 1, u)
            return str((math.ceil(minima) + d) ** 3)[m]


def test():
    D_V = "".join(str(i**3) for i in range(1, 1000))
    for i, c in enumerate(D_V):
        solver = ДимаЗарплата(i + 1)
        if solver.solve() != c:
            return False
    return True


if __name__ == "__main__":
    k = int(input())
    print(ДимаЗарплата(k).solve())
