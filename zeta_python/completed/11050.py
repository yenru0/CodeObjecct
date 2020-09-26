N, K = map(int, input().split())


def C(n, k):
    if n == 1:
        return 1
    elif k == 0 or k == n:
        return 1
    else:
        return C(n - 1, k) + C(n - 1, k - 1)


print(C(N, K))
