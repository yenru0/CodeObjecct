n, m = map(int, input().split())
def solve(ns, m):
    for i in range(1, n+1):
        if i in ns:
            continue
        else:
            if m == 1:
                print(" ".join(map(str, ns+(i,))))
            else:
                solve(ns+(i,), m - 1)
solve((), m)