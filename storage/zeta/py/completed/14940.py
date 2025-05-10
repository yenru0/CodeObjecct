def solve(N, M, X, dest):
    delta = ((-1, 0), (1, 0), (0, 1), (0, -1))
    D = []
    # X = [[-1] * M for _ in range(N)]

    D.append((dest, 0))
    while D:
        now, cost = D.pop(0)
        if X[now[0]][now[1]] == -1:
            X[now[0]][now[1]] = cost

        else:
            continue

        for dx, dy in delta:
            new = now[0] + dx, now[1] + dy
            if 0 <= new[0] < N and 0 <= new[1] < M:
                D.append((new, cost + 1))
    return X


if __name__ == '__main__':
    N, M = map(int, input().split())
    U = []
    X = [[-1] * M for _ in range(N)]
    dest: tuple
    for i in range(N):
        temp = list(map(int, input().split()))
        for j, v in enumerate(temp):
            if v == 0:
                X[i][j] = 0
            elif v == 2:
                dest = (i, j)

    for i in solve(N, M, X, dest):
        print(*i)
