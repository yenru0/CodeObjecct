def LCS(s1, s2) -> int:
    MAXSP = (0, 0, 0)
    DP = [[0 for _ in range(len(s2))] for _ in range(len(s1))]
    for i, c1 in enumerate(s1):
        for j, c2 in enumerate(s2):
            if c1 == c2:
                if i >= 1 and j >= 1:
                    DP[i][j] = DP[i - 1][j - 1] + 1
                else:
                    DP[i][j] = 1
                if MAXSP[0] < DP[i][j]:
                    MAXSP = (DP[i][j], i, j)
    return MAXSP[0]


if __name__ == "__main__":
    s1, s2 = input(), input()
    print(LCS(s1, s2))
