def solve(N: int, S: list[int]):
    S.sort()
    st1: int = 0
    st2: int = st1 + 1
    end: int = N - 1
    r = 1000_0000_0000_0000
    r_pos = None, None, None
    while st2 < end:
        while st2 < end:
            s = S[st1] + S[st2] + S[end]
            if abs(s) < r:
                r = abs(s)
                r_pos = st1, st2, end
                if r == 0:
                    return sorted((S[r_pos[0]], S[r_pos[1]], S[r_pos[2]]))
            if s < 0:
                st2 += 1
            elif s > 0:
                end -= 1

        st1 += 1
        st2 = st1 + 1
        end = N - 1

    return sorted((S[r_pos[0]], S[r_pos[1]], S[r_pos[2]]))


if __name__ == "__main__":
    N = int(input())
    S = list(map(int, input().split()))
    print(*solve(N, S))
