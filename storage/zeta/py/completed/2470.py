def solve_simple(N, S):
    _, s = min(
        [
            (abs(S[i] + S[j]), sorted((S[i], S[j])))
            for i in range(N)
            for j in range(N)
            if i != j
        ]
    )
    return s


def solve(N: int, S: list) -> tuple[int, int]:
    S.sort()
    start = 0
    end = N - 1
    r = 10000000000
    r_pos = None, None
    while start < end:
        s = S[start] + S[end]
        if r > abs(s):
            r = abs(s)
            r_pos = start, end
            if r == 0:
                break
        if s <= 0:
            start += 1
        elif s > 0:
            end -= 1

    return S[r_pos[0]], S[r_pos[1]]


if __name__ == "__main__":
    N = int(input())
    S = list(map(int, input().split()))
    print(*solve(N, S))
