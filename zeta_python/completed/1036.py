import sys

N = int(input())
S = [input() for i in range(N)]
K = int(input())


def change(s):
    return int(s, 36)


def nofet(i):
    if i <= 9:
        return chr(i + 48)
    else:
        return chr(i + 55)


def rechange(i):
    s = ""
    if i == 0:
        return "0"
    while i > 0:
        s += nofet(i % 36)
        i //= 36
    return s[::-1]


mult = [[0, i] for i in range(36)]

for s in S:
    for i, c in enumerate(s[::-1]):
        mult[change(c)][0] += 36 ** i

mult.sort(key=lambda M: M[0] * (35 - M[1]))
print(rechange(sum([mult[i][0] * mult[i][1] for i in range(36 - K)] + [mult[i][0] * 35 for i in range(36 - K, 36)])))
