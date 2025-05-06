from collections import deque
import sys

Map = list()
K = []  # pos list

for i in range(9):
    t = list(map(int, sys.stdin.readline().split()))
    for j in range(9):
        if t[j] == 0:
            K.append((i, j))  # pos
    Map.append(t)

kl = len(K)


def check_possibility(l):
    wbr = []
    for num in range(9, 0, -1):
        if num in Map[K[l][0]]:
            continue
        elif num in (Map[i][K[l][1]] for i in range(9)):
            continue
        tx = K[l][0] // 3 * 3
        ty = K[l][1] // 3 * 3
        if any(num in s[ty:ty + 3] for s in Map[tx:tx + 3]):
            continue
        wbr.append(num)
    return wbr


T = deque()
for i in check_possibility(0):
    T.append((i, 0))

beforeDepth = 0

while T:
    now, d = T.pop()
    if d >= beforeDepth:
        Map[K[d][0]][K[d][1]] = now
    else:
        for i in range(d + 1, beforeDepth + 1):
            Map[K[i][0]][K[i][1]] = 0
        Map[K[d][0]][K[d][1]] = now
    if d == kl - 1:
        break
    for i in check_possibility(d + 1):
        T.append((i, d + 1))
    beforeDepth = d
for row in Map:
    print(" ".join(map(str, row)))
