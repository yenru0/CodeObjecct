def dfs(N, M, E, V):
    D = []
    D.append((V))
    visited = []
    while D:
        now = D.pop()
        if now in visited:
            continue
        else:
            visited.append(now)
        for i in E[now][::-1]:
            D.append(i)

    return visited


def bfs(N, M, E, V):
    D = []
    D.append((V))
    visited = []
    while D:
        now = D.pop(0)
        if now in visited:
            continue
        else:
            visited.append(now)
        for i in E[now]:
            D.append(i)

    return visited


if __name__ == '__main__':
    N, M, V = map(int, input().split())
    E = {}
    for i in range(1, N + 1):
        E[i] = []
    for i in range(1, M + 1):
        a, b = map(int, input().split())
        E[a].append(b)
        E[b].append(a)
    for i in range(1, N + 1):
        E[i].sort()
    print(*dfs(N, M, E, V))
    print(*bfs(N, M, E, V))
