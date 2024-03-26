def check(U):
    for i in U:
        for j in i:
            for k in j:
                if k == 0:
                    return False
    return True


def solve(M, N, H, U, ir):
    step = 0
    delta = ((-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1))
    rotten = [ir]
    while not check(U):
        rotten.append([])
        for r in rotten[step]:
            for dm, dn, dh in delta:
                new = r[0] + dm, r[1] + dn, r[2] + dh
                if 0 <= new[0] < M and 0 <= new[1] < N and 0 <= new[2] < H:
                    if U[new[2]][new[1]][new[0]] != 0:
                        continue
                    else:
                        U[new[2]][new[1]][new[0]] = 1
                        rotten[step + 1].append(new)
        if rotten[-1]:
            step += 1
        else:
            return -1

    return step


if __name__ == '__main__':
    M, N, H = map(int, input().split())
    U = [[[None for _ in range(M)] for _ in range(N)] for _ in range(H)]  # U[h][n][m]
    init_rottens = []
    for h in range(H):
        for n in range(N):
            for m, v in enumerate(map(int, input().split())):
                U[h][n][m] = v
                if v == 1:
                    init_rottens.append((m, n, h))

    print(solve(M, N, H, U, init_rottens))
