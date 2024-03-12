def solve(N: int, W: list):
    V = {}

    for w in W:
        for i, a in enumerate(w[::-1]):
            if a not in V:
                V[a] = 0
            V[a] += 10 ** i
    S = list(sorted(V.items(), key=lambda x: x[1], reverse=True))
    return sum([(9 - i) * S[i][1] for i in range(len(S))])


if __name__ == '__main__':
    N = int(input())
    W = [input() for _ in range(N)]
    print(solve(N, W))
