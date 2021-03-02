def solve(N: int, A: list):
    T = [[0, 0] for _ in range(N)]
    T[0][0] = 1  # 상승부
    T[0][1] = 1  # 하강부

    for i in range(1, N):  # mainloop
        t = []
        for j in range(i):
            if A[j] < A[i]:
                t.append(T[j][0] + 1)
        else:
            t.append(1)
        T[i][0] = max(t)

        t = []
        for j in range(i):
            if A[j] > A[i]:
                t.append(T[j][0] + 1)
                t.append(T[j][1] + 1)
        else:
            t.append(1)
        T[i][1] = max(t)
    return max(max(T, key=lambda x: max(x)))


if __name__ == '__main__':
    print(solve(int(input()), list(map(int, input().split()))))
