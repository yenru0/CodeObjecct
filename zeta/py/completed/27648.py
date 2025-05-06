if __name__ == "__main__":
    N, M, K = map(int, input().split())
    S = []
    for i in range(N):
        S.append([])
        for j in range(M):
            S[-1].append(j + i + 1)
    if S[-1][-1] <= K:
        print("YES")
        for i in range(N):
            print(" ".join(map(str, S[i])))

    else:
        print("NO")
