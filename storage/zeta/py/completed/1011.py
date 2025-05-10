import math
T = int(input())

for i in range(T):
    x, y = map(int, input().split())
    distance = y - x
    disted = distance ** (1 / 2)
    upper = math.ceil(disted)

    lowersq = (upper - 1)**2
    uppersq = upper**2

    if (lowersq + uppersq)/2 <= distance:
        print(upper * 2 -1)
    else:
        print(upper * 2 - 2)
