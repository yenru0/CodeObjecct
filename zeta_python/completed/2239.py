import sys

Map = list()
K = []  # pos list

for i in range(9):
    t = list(map(int, input()))
    for j in range(9):
        if t[j] == 0:
            K.append((i, j))  # pos
    Map.append(t)

kl = len(K)


def check_possibility(l):
    wbr = []
    for num in range(1, 10):
        if num in Map[K[l][0]]:
            continue
        elif num in [Map[i][K[l][1]] for i in range(9)]:
            continue
        tx = K[l][0] // 3 * 3
        ty = K[l][1] // 3 * 3
        if any(num in s[ty:ty + 3] for s in Map[tx:tx + 3]):
            continue
        wbr.append(num)
    return wbr


T = []
for i in check_possibility(0)[::-1]:
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
    for i in check_possibility(d + 1)[::-1]:
        T.append((i, d + 1))
    beforeDepth = d
for row in Map:
    for col in row:
        print(col, end='')
    print()

# ㄹㅇ 재귀를 써야 하는거신가? 일단 함수를 적합도 함수를 손봐주볼까?
