if __name__ == "__main__":
    N, K = map(int, input().split())
    L = list(map(int, input().split()))
    M = []

    for i in range(N):
        if L[i] == 1:
            M.append(i)

    print(min((M[i + K - 1] - M[i] + 1 for i in range(len(M) - K + 1)), default=-1))
