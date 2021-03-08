def get_2_5(_to):
    c2, c5 = 0, 0

    k = 1
    while k <= _to:
        k *= 2
        c2 += _to // k
    k = 1
    while k <= _to:
        k *= 5
        c5 += _to // k

    return c2, c5


def solve(N, M):
    count_2, count_5 = get_2_5(N)
    t = get_2_5(N - M)
    count_2 -= t[0]
    count_5 -= t[1]
    t = get_2_5(M)
    count_2 -= t[0]
    count_5 -= t[1]
    return min((count_2, count_5))


if __name__ == '__main__':
    print(solve(*map(int, input().split())))
