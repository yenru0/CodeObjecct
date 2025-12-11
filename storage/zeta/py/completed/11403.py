import sys

input = sys.stdin.readline
write = sys.stdout.write

THRESHOLD = 10**9

def floyd_warshall(n: int, adj: list[list[int]]):
    ds = []
    ds.append(adj)
    for k in range(n):
        mat = [[0] * n for _ in range(n)]
        for i in range(n):
            for j in range(n):
                if ds[-1][i][k] >= THRESHOLD or ds[-1][k][j] >= THRESHOLD:
                    mat[i][j] = ds[-1][i][j]
                else:
                    mat[i][j] = min(ds[-1][i][j], ds[-1][i][k] + ds[-1][k][j])
        ds.append(mat)
    return ds[-1]

if __name__ == '__main__':
    n = int(input())
    adj = [list(map(lambda x: THRESHOLD if int(x) == 0 else int(x), input().split())) for _ in range(n)]
    result = floyd_warshall(n, adj)
    for row in result:
        write(' '.join(map(lambda x: '0' if x >= THRESHOLD else '1', row)) + '\n')
    
