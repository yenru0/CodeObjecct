def solve(N, H):
    lasts = []
    for i, h in enumerate(H):
        for j, l in enumerate(lasts):
            if h == l - 1:
                lasts[j] -= 1
                break
        else:
            lasts.append(h)

    return len(lasts)


if __name__ == '__main__':
    N = int(input())
    H = list(map(int, input().split()))
    print(solve(N, H))
