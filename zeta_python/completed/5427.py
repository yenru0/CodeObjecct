import sys
from collections import deque

input = sys.stdin.readline


def translate(char):
    if char == '.':
        return 0
    elif char == '#':
        return 1
    elif char == '*':
        return 2
    else:
        return 3


def solve(w, h, linelist):
    Map = [[0] * h for _ in range(w)]
    firepoint = []
    startpoint = []

    # preprocess
    for i, line in enumerate(linelist):
        for j, c in enumerate(line):
            t = translate(c)
            if t == 2:
                firepoint.append((j, i))
            elif t == 3:
                startpoint.append((j, i))
            Map[j][i] = t

    # run
    step = 0
    D = deque()
    delta = ((-1, 0), (1, 0), (0, -1), (0, 1))
    for fp in firepoint:
        D.append((fp, 2))
    for sp in startpoint:
        D.append((sp, 3))
    while True:
        step += 1
        firepoint = []
        startpoint = []
        while D:
            point, selector = D.popleft()
            for dx, dy in delta:
                new = point[0] + dx, point[1] + dy
                if 0 <= new[0] < w and 0 <= new[1] < h:
                    if Map[new[0]][new[1]] != 0:
                        continue
                    else:
                        Map[new[0]][new[1]] = selector
                        if selector == 2:
                            firepoint.append(new)
                        else:
                            startpoint.append(new)

                else:
                    if selector == 3:
                        return step
                    else:
                        continue

        for fp in firepoint:
            D.append((fp, 2))

        if not startpoint:
            return None
        for sp in startpoint:
            D.append((sp, 3))


if __name__ == '__main__':
    N: int = int(input())
    for _ in range(N):
        w, h = map(int, input().split())
        result = solve(w, h, [input().strip() for _ in range(h)])

        if result is None:
            print("IMPOSSIBLE")
        else:
            print(result)
