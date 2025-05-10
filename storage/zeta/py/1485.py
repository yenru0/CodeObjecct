N = int(input())


def check90(p1, p2, p3):
    if p2[0] - p1[0] == 0:
        if p3[1] - p1[1] == 0:
            return True
    elif p3[0] - p1[0] == 0:
        if p2[1] - p1[1] == 0:
            return True

    return - (p3[0] - p1[0]) * (p2[1] - p1[1]) == (p3[1] - p1[1]) * (p2[0] - p1[0])


for _ in range(N):
    P = [(p1x, p1y), (p2x, p2y), (p3x, p3y), (p4x, p4y)] = \
        map(int, input().split()), map(int, input().split()), map(int, input().split()), map(int, input().split())
    if check90((p1x, p1y), (p2x, p2y), (p3x, p3y)):
        (opox, opoy) = p4x, p4y
        (remx, remy) = p3x, p3y
    elif check90((p1x, p1y), (p2x, p2y), (p4x, p4y)):
        (opox, opoy) = p3x, p3y
        (remx, remy) = p2x, p2y
    elif check90((p1x, p1y), (p3x, p3y), (p4x, p4y)):
        (opox, opoy) = p2x, p2y
        (remx, remy) = p4x, p4y
    else:
        print(0)
        continue
    if (opox - p1x) ** 2 + (opoy - p1y) ** 2 == 2 * (remx - p1x) ** 2 + 2 * (remy - p1y) ** 2:
        print(1)
    else:
        print(0)
