Mem = [[[None for _ in range(50)] for _ in range(50)] for _ in range(50)]


def w(a, b, c):
    if a <= 0 or b <= 0 or c <= 0:
        return 1
    elif Mem[a - 1][b - 1][c - 1] is not None:
        return Mem[a - 1][b - 1][c - 1]
    t = 0

    if a > 20 or b > 20 or c > 20:
        t = w(20, 20, 20)
    elif a < b < c:
        t = w(a, b, c - 1) + w(a, b - 1, c - 1) - w(a, b - 1, c)
    else:
        t = w(a - 1, b, c) + w(a - 1, b - 1, c) + w(a - 1, b, c - 1) - w(a - 1, b - 1, c - 1)

    Mem[a - 1][b - 1][c - 1] = t
    return t


while True:
    a, b, c = map(int, input().split())
    if a == b == c == -1:
        break
    print(f"w({a}, {b}, {c}) = {w(a, b, c)}")
