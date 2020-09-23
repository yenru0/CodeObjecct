import sys

N = int(input())
SR = []

T = []
TM = 0


def getTopLeft(c4):
    return c4[0], c4[-1]


def getBottomRight(c4):
    return c4[2], c4[1]


S = [tuple(map(int, sys.stdin.readline().split())) for i in range(N)]

for c4 in S:
    m = getBottomRight(c4)
    dm = m[1] / m[0]
    M = getTopLeft(c4)
    dM = M[1] / M[0]
    SR.append((dm, dM))

for i in SR:
    cnt = 0
    cnt2 = 0
    for j in SR:
        if j[0] <= i[0] <= j[1]:
            cnt += 1
        if j[0] <= i[1] <= j[1]:
            cnt2 += 1
    if TM < cnt:
        TM = cnt
    if TM < cnt2:
        TM = cnt2

print(TM)
