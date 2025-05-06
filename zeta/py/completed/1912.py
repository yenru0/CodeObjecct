def solve(N, A):
    T = [0] * N
    cum = [0] * N
    T[0] = A[0]
    cum[0] = A[0]
    for i in range(1, N):
        cumsum = A[i] + cum[i - 1]
        if T[i - 1] > cumsum:  # condition
            if T[i - 1] < 0:
                cum[i] = T[i] = max((A[i], T[i - 1]))
            else:
                cum[i] = max((cumsum, A[i]))  # if ordinary
                T[i] = T[i - 1]
        else:
            T[i] = cum[i] = max((cumsum, A[i]))
    return T[-1]


if __name__ == '__main__':
    print(solve(int(input()), list(map(int, input().split()))))
