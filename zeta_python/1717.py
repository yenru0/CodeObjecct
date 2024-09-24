import sys

input = sys.stdin.readline


def root(P, e):
    node = e
    while node != P[e]:
        node = P[e]
    return node


if __name__ == "__main__":
    N, M = map(int, input().split())
    P = [i for i in range(N + 1)]
    for _ in range(M):
        op, a, b = map(int, input().split())
        if op == 0:  # Merge
            if a == b:
                continue
            rb, ra = root(P, b), root(P, a)
            if rb != ra:
                P[rb] = ra
            print(P)
        elif op == 1:  # Find
            print("YES" if a == b or root(P, a) == root(P, b) else "NO")
