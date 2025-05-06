def S(n: int):
    s = n & 1
    for i in range(60 - 1, 0, -1):
        if n & (1 << i):
            s += S.points[i - 1] + (n - (1 << i) + 1)
            n -= 1 << i
    return s


S.points = [0] * 60
S.points[0] = 1
for i in range(1, 60):
    S.points[i] = 2 * S.points[i - 1] + (1 << i)

if __name__ == "__main__":
    A, B = map(int, input().split())
    print(S(B) - S(A - 1))
