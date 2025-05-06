def solve(N, P):
    P.sort()
    S = P[0]
    before = P[0]
    for p in P[1:]:
        S += before + p
        before += p
    return S


if __name__ == '__main__':
    print(solve(int(input()), list(map(int, input().split()))))
