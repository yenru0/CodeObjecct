import sys

input = sys.stdin.readline


def longest(E, start) -> tuple[int, int]:
    D = []

    M = 0
    V_M = 0
    D.append((start, [start], 0))  # now, visited, accumulated

    while D:
        now, visited, accumulated = D.pop(0)
        flag = True
        for e in E[now]:
            target, cost = e[0], e[1]
            if target not in visited:
                flag = False
                D.append((target, visited + [target], cost + accumulated))

        if flag:
            if accumulated > M:
                M = accumulated
                V_M = now

    return V_M, M


def solve(V, E):
    first = longest(E, 1)[0]
    _, diameter = longest(E, first)

    return diameter


if __name__ == "__main__":
    V = int(input())
    E = {}
    for _ in range(V):
        _i = list(map(int, input().split()))
        E[_i[0]] = []

        for i, v in zip(_i[1:-1:2], _i[2:-1:2]):
            E[_i[0]].append((i, v))

    print(solve(V, E))
