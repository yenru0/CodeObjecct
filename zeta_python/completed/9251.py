def solve(s1, s2):
    N1 = len(s1)
    N2 = len(s2)
    T = [[0 for j in range(N2 + 1)] for i in range(N1 + 1)]
    for i in range(1, N1 + 1):
        for j in range(1, N2 + 1):
            if s1[i - 1] == s2[j - 1]:
                T[i][j] = T[i - 1][j - 1] + 1
            else:
                T[i][j] = max((T[i][j - 1], T[i - 1][j]))
    return T[-1][-1]


if __name__ == '__main__':
    print(solve(input(), input()))
