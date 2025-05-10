from bisect import bisect_left


def solve(N, A, B, Q, W):
    S = []
    before = 100000000000000000000
    for i, ab in enumerate(zip(A, B)):
        now = ab[0] - ab[1]
        if now > before:
            continue
        elif now == before:
            S.append((now, i + 1))
        else:
            S.append((now, i + 1))
            before = now

    R = []
    rS = S[::-1]

    for w in W:
        pos = bisect_left(rS, w, key=lambda x: x[0])
        if len(S) <= pos:
            R.append(0)
            continue
        if rS[pos][0] == w:
            if pos == 0:
                R.append(N)
            else:
                R.append(rS[pos - 1][1] - 1)
        else:
            if pos == 0:
                R.append(N)
            else:
                R.append(rS[pos - 1][1] - 1)
    return R


if __name__ == "__main__":
    N = int(input())
    A = list(map(int, input().split()))
    B = list(map(int, input().split()))
    Q = int(input())
    W = list(map(int, input().split()))
    print(*solve(N, A, B, Q, W), sep="\n")
