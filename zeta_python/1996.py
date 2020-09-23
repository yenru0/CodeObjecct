N = int(input())

Map = [[0] * (N + 2)]
Nap = [[0] * N for i in range(N)]


def sumAround(x, y):
    return Map[x - 1][y - 1] + Map[x - 1][y] + Map[x - 1][y + 1] + Map[x][y - 1] + Map[x][y + 1] + Map[x + 1][y - 1] + \
           Map[x + 1][y] + Map[x + 1][y + 1]


for i in range(N):
    Map.append([0] + [0 if x == "." else int(x) for x in input()] + [0])

Map.append([0] * (N + 2))

for i in range(1, N + 1):
    for j in range(1, N + 1):
        t = Map[i][j]
        if t != 0:
            Nap[i - 1][j - 1] = "*"
        else:
            k = sumAround(i, j)
            Nap[i - 1][j - 1] = "M" if k > 9 else k

for i in Nap:
    for j in i:
        print(j, end='')
    print()
