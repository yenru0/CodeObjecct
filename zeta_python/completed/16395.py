def C(n, k):
    if C.M[n][k]:
        return C.M[n][k]
    elif k == 0 or k == n:
        tmp = 1
    else:
        tmp = C(n - 1, k) + C(n - 1, k - 1)
    C.M[n][k] = tmp
    return tmp


C.M = [[0] * (i + 1) for i in range(30)]
C.M[0] = [1]
C.M[1] = [1, 1]

if __name__ == "__main__":
    print(C(*map(lambda x: int(x) - 1, input().split())))
