def solve(A, B, C) -> int:
    A %= C
    ret = 1
    while B:
        if B % 2 == 1:
            ret *= A
            ret %= C
        A *= A
        A %= C
        B //= 2

    return ret


if __name__ == "__main__":
    A, B, C = map(int, input().split())
    print(solve(A, B, C))
