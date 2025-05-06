def solve(N, K):
    Kp = (K * (K + 1)) // 2
    if N < Kp:
        return -1
    else:
        N -= Kp
        r = N % K
        return sum((K - 1, 1 if r > 0 else 0))


if __name__ == '__main__':
    N, K = map(int, input().split())
    print(solve(N, K))
