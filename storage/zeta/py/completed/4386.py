import sys

input = sys.stdin.readline


def dist(v1: tuple, v2: tuple) -> float:
    return ((v1[0] - v2[0]) ** 2 + (v1[1] - v2[1]) ** 2) ** 0.5


def find(conn, target):
    a = target
    while a != conn[a]:
        a = conn[a]

    return a


def union(conn, a, b):
    ra, rb = find(conn, a), find(conn, b)
    if ra < rb:
        conn[rb] = ra
    else:
        conn[ra] = rb


def get_MST(N: int, Es: list[tuple]) -> float:
    Es.sort(key=lambda x: x[-1])
    connected = [i for i in range(N)]

    cnt = 0
    s = 0
    for a, b, w in Es:
        if find(connected, a) != find(connected, b):
            union(connected, a, b)
            s += w
            cnt += 1
            if cnt == N - 1:
                break
    return s


if __name__ == "__main__":
    N = int(input())
    Vs = list(tuple(map(float, input().split())) for _ in range(N))

    available_lines = [
        (i, j, dist(Vs[i], Vs[j])) for i in range(N - 1) for j in range(i + 1, N)
    ]

    print(get_MST(N, available_lines))
