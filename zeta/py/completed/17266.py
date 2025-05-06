if __name__ == "__main__":
    N, M = int(input()), int(input())
    X = list(map(int, input().split()))

    max_dx = max([X[i + 1] - X[i] for i in range(M - 1)], default=0)
    max_h = max_dx // 2 if max_dx % 2 == 0 else max_dx // 2 + 1

    print(max([max_h, N - X[-1], X[0]]))
