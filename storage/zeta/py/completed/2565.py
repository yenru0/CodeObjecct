def solve(N, R):
    R.sort()
    U = [0 for i in range(N)]
    U[0] = 1
    for i in range(N):
        t_u = []
        for j in range(i):
            if R[j][1] < R[i][1]:  # up
                t_u.append(U[j] + 1)
        else:
            t_u.append(1)
        U[i] = max(t_u)
    return N - max(U)


if __name__ == '__main__':
    N = int(input())
    print(solve(N, [list(map(int, input().split())) for _ in range(N)]))
