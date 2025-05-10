# Px+Qy >= D
def solve(D, P, Q):
    if D % P == 0 or D % Q == 0:
        return D
    Q, P = sorted((P, Q))
    p_bound = (D // P) + 1

    min_S = 10 ** 13
    for x in range(0, p_bound + 1):

        d, m = divmod(D - P * x, Q)
        if m == 0:
            return D
        new_S = (d + 1) * Q + P * x
        if new_S < min_S:
            min_S = new_S
            if min_S == D:
                return min_S
    return min_S


if __name__ == '__main__':
    D, P, Q = map(int, input().split())
    print(solve(D, P, Q))
