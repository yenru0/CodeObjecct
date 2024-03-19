def solve(N, X, E):
    ComputerMap = {}
    WormMap = [0 for i in range(N + 1)]
    WormMap[1] = 1
    for i in range(1, N + 1):
        ComputerMap[i] = []
    for e in E:
        ComputerMap[e[0]].append(e[1])
        ComputerMap[e[1]].append(e[0])

    D = []
    D.append(1)
    while D:
        x = D.pop()
        connected = ComputerMap[x]
        for conn in connected:
            if WormMap[conn]:
                continue
            else:
                WormMap[conn] = 1
                D.append(conn)

    return sum(WormMap) - 1


if __name__ == '__main__':
    N = int(input())
    X = int(input())
    E = list(sorted(list(map(int, input().split()))) for _ in range(X))
    print(solve(N, X, E))
