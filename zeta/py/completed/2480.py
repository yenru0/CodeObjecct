I = list(map(int, input().split()))


def reward():
    global I
    M = 0
    for i in range(1, 10):
        if I.count(i) == 1:
            M = i
        elif I.count(i) == 2:
            return 1000 + i * 100
        elif I.count(i) == 3:
            return 10000 + 1000 * i
    return M * 100


print(reward())
