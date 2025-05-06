DIVIDER = 10**9 + 7


def progress(m):
    return (
        (m[1] + m[2] + m[3] + m[4]) % DIVIDER,
        (m[3] + m[4] + m[0]) % DIVIDER,
        (m[4] + m[0]) % DIVIDER,
        (m[1] + m[0]) % DIVIDER,
        (m[1] + m[2] + m[0]) % DIVIDER,
    )


if __name__ == "__main__":
    M = (1, 1, 1, 1, 1)
    # M2 = (1 + 1 + 1, )
    N = int(input())
    for _ in range(1, N):
        M = progress(M)

    print(sum(M) % DIVIDER)
