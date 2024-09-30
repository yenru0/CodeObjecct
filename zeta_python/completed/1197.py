import sys

input = sys.stdin.readline


def find(P, a) -> int:
    node = a
    while node != P[node]:
        node = P[node]
    return node


def union(P, a, b) -> None:
    ra = find(P, a)
    rb = find(P, b)
    if ra < rb:
        P[rb] = ra
    else:
        P[ra] = rb


def two_of_them(P, a, b) -> bool:
    return find(P, a) == find(P, b)


def get_MST(V, E, Es) -> int:
    # sort Edges
    Es.sort(key=lambda x: x[-1])

    P = [i for i in range(V + 1)]
    cum_edges = []
    for ei in range(E):
        s, e, w = Es[ei]
        if not two_of_them(P, s, e):
            cum_edges.append(w)
            union(P, s, e)
            if len(cum_edges) == V - 1:
                break

    return sum(cum_edges)


if __name__ == "__main__":
    V, E = map(int, input().split())
    Es = []
    for _ in range(E):
        start, end, weight = map(int, input().split())
        Es.append((start, end, weight))

    print(get_MST(V, E, Es))
