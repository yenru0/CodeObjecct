def find_group(Map, M, N, i, j):
    ret = []
    temp = []
    temp.append((i, j))
    while temp:
        t = temp.pop()
        if t in ret:
            continue
        if 1 <= t[0]:
            if Map[t[0] - 1][t[1]]:
                temp.append((t[0] - 1, t[1]))
        if t[0] < M - 1:
            if Map[t[0] + 1][t[1]]:
                temp.append((t[0] + 1, t[1]))
        if 1 <= t[1]:
            if Map[t[0]][t[1] - 1]:
                temp.append((t[0], t[1] - 1))
        if t[1] < N - 1:
            if Map[t[0]][t[1] + 1]:
                temp.append((t[0], t[1] + 1))
        ret.append(t)
    return ret


def solve(M, N, Ks):
    Map = [[0 for _ in range(N)] for _ in range(M)]
    count = 0
    found = {}
    for k in Ks:
        Map[k[0]][k[1]] = 1

    for i in range(M):
        for j in range(N):
            if Map[i][j]:
                if (i, j) in found:
                    continue
                else:
                    r = find_group(Map, M, N, i, j)
                    count += 1
                    for s in r:
                        found[s] = count
    return count


if __name__ == '__main__':
    T = int(input())
    for _ in range(T):
        M, N, K = map(int, input().split())
        Ks = [tuple(map(int, input().split())) for _ in range(K)]
        print(solve(M, N, Ks))
