import sys
import math

input = sys.stdin.readline


class SprinklerArea:
    _T_ONCE = 1 / 4 * math.pi
    _T_DUET = 1 / 6 * math.pi + 3 ** (1 / 2) / 4

    def __init__(self, size: tuple[int, int], n: int, sprinkler_pos):
        self.size = size
        self.map = [[0 for _ in range(size[1])] for _ in range(size[0])]

        self.n = n
        self.sprinkler_pos = sprinkler_pos

    @staticmethod
    def combine(r_before, r_after):
        if r_before == 0:
            return r_after
        elif r_before in [1, 2, 3, 4]:
            if r_after % 2 == r_before % 2:
                return 6
            else:
                return 5
        else:
            return 6

    @staticmethod
    def get_area(type: int):
        if type == 0:
            return 0.0
        elif type in [1, 2, 3, 4]:
            return SprinklerArea._T_ONCE
        elif type == 5:
            return SprinklerArea._T_DUET
        else:
            return 1.0

    def solve(self):
        for pos_x, pos_y in self.sprinkler_pos:
            calc_pos = []
            if pos_x == 0:
                if pos_y == 0:
                    calc_pos.append(((pos_x, pos_y), 2))
                elif pos_y == self.size[1]:
                    calc_pos.append(((pos_x, pos_y - 1), 3))
                else:
                    calc_pos.append(((pos_x, pos_y), 2))
                    calc_pos.append(((pos_x, pos_y - 1), 3))
            elif pos_x == self.size[0]:
                if pos_y == 0:
                    calc_pos.append(((pos_x - 1, pos_y), 1))
                elif pos_y == self.size[1]:
                    calc_pos.append(((pos_x - 1, pos_y - 1), 4))
                else:
                    calc_pos.append(((pos_x - 1, pos_y), 1))
                    calc_pos.append(((pos_x - 1, pos_y - 1), 4))
            else:
                if pos_y == 0:
                    calc_pos.append(((pos_x - 1, pos_y), 1))
                    calc_pos.append(((pos_x, pos_y), 2))
                elif pos_y == self.size[1]:
                    calc_pos.append(((pos_x - 1, pos_y - 1), 4))
                    calc_pos.append(((pos_x, pos_y - 1), 3))
                else:
                    calc_pos.append(((pos_x - 1, pos_y), 1))
                    calc_pos.append(((pos_x - 1, pos_y - 1), 4))
                    calc_pos.append(((pos_x, pos_y), 2))
                    calc_pos.append(((pos_x, pos_y - 1), 3))
            for pos, rplus in calc_pos:
                self.map[pos[0]][pos[1]] = self.combine(self.map[pos[0]][pos[1]], rplus)

        return sum(sum(map(self.get_area, row)) for row in self.map)


if __name__ == '__main__':
    size = tuple(map(int, input().split()))
    n = int(input())
    pos = [tuple(map(int, input().split())) for _ in range(n)]
    solver = SprinklerArea(size, n, pos)
    print(solver.solve())
