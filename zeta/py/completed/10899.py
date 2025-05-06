if __name__ == "__main__":
    P, N = map(int, input().split())
    S = list(map(int, input().split()))
    S.sort(reverse=True)
    I = []
    P -= 1
    for _ in range(N):
        s = S.pop()
        if P - s < 0:
            break
        else:
            I.append(P)
            P -= s

    print(len(I), sum(I))
