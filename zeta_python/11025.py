def solve(N, K):
    ret = 1
    for i in range(2, N + 1):
        ret = (ret + K - 1) % i + 1
    return ret


if __name__ == "__main__":
    print(solve(*map(int, input().split())))
