def solve(N, E):
    D = []
    D.append((0, 1, 0))
    finals = []
    while D:
        before, header, distance = D.pop(0)
        nexts = E[header]
        cnt = 0
        for s in nexts:
            if s[0] != before:
                D.append((header, s[0], distance + s[1]))
                cnt += 1
        if cnt == 0:
            finals.append((header, distance))

    D = []
    s1 = max(finals, key=lambda x: x[1])
    D.append((0, s1[0], 0))
    finals = []
    while D:
        before, header, distance = D.pop(0)
        nexts = E[header]
        cnt = 0
        for s in nexts:
            if s[0] != before:
                D.append((header, s[0], distance + s[1]))
                cnt += 1
        if cnt == 0:
            finals.append((header, distance))

    return max(finals, key=lambda x: x[1])[1]


if __name__ == '__main__':
    N: int = int(input())
    E = [[] for _ in range(N + 1)]
    for a, b, v in [map(int, input().split()) for _ in range(N - 1)]:
        E[a].append((b, v))
        E[b].append((a, v))
    print(solve(N, E))
