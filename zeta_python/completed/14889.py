N = int(input())
S = [list(map(int, input().split())) for _ in range(N)]  # S[j][i]

T = [0 for i in range(N)]
U = []


def get_score(i, j):
    return S[i][j] + S[j][i]


def get_scores(p):
    score = 0
    for i in range(N // 2):
        for j in range(i + 1, N // 2):
            score += get_score(p[i], p[j])
    return score


def f(t, l):
    T[t] = 1
    if l == N // 2 - 1:
        ut1, ut2 = [], []
        for i, v in enumerate(T):
            if v == 0:
                ut1.append(i)
            else:
                ut2.append(i)
        u1 = get_scores(ut1)
        u2 = get_scores(ut2)
        # print(T, ut1, ut2, u1, u2)
        U.append(abs(u1 - u2))
        return

    for i in range(t + 1, N):
        T[i] = 1
        f(i, l + 1)
        T[i] = 0


f(0, 0)
print(min(U))
