if __name__ == "__main__":
    N = int(input())
    X = list(map(int, input().split()))
    if N == 1:
        R = [0]
    else:
        R = list(map(int, input().split())) + [0]
    Y = []
    Y.append((X[0], R[0]))
    for i in range(1, N):
        if Y[-1][0] == X[i]:
            if Y[-1][1] < R[i]:
                Y.pop()
                Y.append((X[i], R[i]))
        else:
            Y.append((X[i], R[i]))

    flag = True
    max_length = 0

    for i in range(len(Y) - 1):
        max_length = max([max_length, sum(Y[i])])
        if max_length >= Y[i+1][0]:
            continue
        else:
            flag = False
            break
    else:
        flag = True

    (
        print("권병장님, 중대장님이 찾으십니다")
        if flag
        else print("엄마 나 전역 늦어질 것 같아")
    )
