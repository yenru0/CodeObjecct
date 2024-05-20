import sys

input = sys.stdin.readline


def color_reversal(c: int) -> int:
    if c == 1:
        return 2
    else:
        return 1


def solve(V: int, E) -> str:

    D = list(range(1, V + 1))
    colored = [0] * (V + 1)
    while D:
        now = D.pop()
        if not colored[now]:
            colored[now] = 1
        for target in E[now]:
            if colored[target]:
                if color_reversal(colored[now]) == colored[target]:
                    continue
                else:
                    return "NO"
            else:
                colored[target] = color_reversal(colored[now])
                D.append(target)
    return "YES"


if __name__ == "__main__":
    K = int(input())
    for _ in range(K):
        V, C = map(int, input().split())
        E = [[] for _ in range(V + 1)]
        for _ in range(C):
            u, v = map(int, input().split())
            E[u].append(v)
            E[v].append(u)
        print(solve(V, E))
