import sys

input = sys.stdin.readline


def check(U):
    for i in U:
        for j in i:
            if j == 0:
                return False
    return True


def solve(M, N, U, ir):
    step = 0
    delta = (
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    )
    rotten = [ir]
    while not check(U):
        rotten.append([])
        for r in rotten[step]:
            for dm, dn in delta:
                new = r[0] + dm, r[1] + dn
                if 0 <= new[0] < M and 0 <= new[1] < N:
                    if U[new[1]][new[0]] != 0:
                        continue
                    else:
                        U[new[1]][new[0]] = 1
                        rotten[step + 1].append(new)
        if rotten[-1]:
            step += 1
        else:
            return -1

    return step


if __name__ == "__main__":
    M, N = map(int, input().split())
    U = [[None for _ in range(M)] for _ in range(N)]  # U[h][n][m]
    init_rottens = []

    for n in range(N):
        for m, v in enumerate(map(int, input().split())):
            U[n][m] = v
            if v == 1:
                init_rottens.append((m, n))

    print(solve(M, N, U, init_rottens))
