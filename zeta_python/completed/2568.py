from bisect import bisect_left


# LIS
def solve(N: int, I: list[tuple[int, int]]) -> tuple[int, list]:
    left = []
    right = []
    D = []
    L = []
    R = []
    I = sorted(I, key=lambda x: x[0])
    for l, r in I:
        left.append(l)
        right.append(r)

    L.append(right[0])
    D.append(left[0])
    R.append((0, 0))
    for i in range(1, N):
        pos = bisect_left(L, right[i])
        if pos >= len(L):
            L.append(right[i])
            D.append(left[i])
        else:
            L[pos] = right[i]
            D[pos] = left[i]
        R.append((pos, i))

    target = len(L) - 1
    FL = []
    for i, j in R[::-1]:
        if i == target:
            FL.append(left[j])
            target -= 1

    return N - len(L), set(left).difference(set(FL))


if __name__ == "__main__":
    N = int(input())
    I = [tuple(map(int, input().split())) for _ in range(N)]

    cnt, L = solve(N, I)
    print(cnt)
    [print(i) for i in L]


# 1 2 3 4 6 7 9 10
# 8 2 9 1 4 6 7 10
# 1 1 2 1 2 3 4 5

# 1 1 2 1 2 3
# 8 2 9 1 4 6
