if __name__ == "__main__":
    N = int(input())
    A = list(map(int, input().split()))
    AS = sum(A)
    S = 0
    for _ in range(N - 1):
        r = A.pop()
        AS -= r
        S += r * AS
    print(S)
