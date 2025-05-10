def get_kevin(N, p, E):
    D = []
    D.append((p, 0))
    kevins = [-1 for _ in range(N + 1)]
    while D:
        now, cost = D.pop(0)
        if kevins[now] == -1:
            kevins[now] = cost
        else:
            continue

        for arrow in E[now]:
            D.append((arrow, cost + 1))
    return sum(kevins)


def get_lowest_kevin(N, E):
    return min([(get_kevin(N, i, E), i) for i in range(1, N + 1)])[1]


if __name__ == '__main__':
    N, M = map(int, input().split())
    E = [[] for _ in range(N + 1)]
    for _ in range(M):
        i, j = map(int, input().split())
        E[i].append(j)
        E[j].append(i)
    print(get_lowest_kevin(N, E))
