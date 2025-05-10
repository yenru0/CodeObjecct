import sys

sys.setrecursionlimit(1 << 16)


def solve(M, N, U):
    """get count of pathes"""

    Mem = [[-1 for _ in range(N)] for _ in range(M)]
    Mem[M - 1][N - 1] = 1

    def dfs(now):
        x, y = now
        if Mem[x][y] != -1:
            return Mem[x][y]
        cnt = 0
        for i, j in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            _x, _y = x + i, y + j
            if 0 <= _x < M and 0 <= _y < N and U[_x][_y] < U[x][y]:
                cnt += dfs((_x, _y))

        Mem[x][y] = cnt
        return Mem[x][y]

    dfs((0, 0))
    return Mem[0][0]


if __name__ == '__main__':
    M, N = map(int, input().split())
    U = [list(map(int, input().split())) for _ in range(M)]  # U[m][n]
    print(solve(M, N, U))
