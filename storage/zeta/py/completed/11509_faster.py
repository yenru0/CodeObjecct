def solve(N, H):
    arrow = [0] * 1000001
    for h in H:
        if arrow[h]:
            arrow[h] -= 1
        arrow[h - 1] += 1
    return sum(arrow)


if __name__ == '__main__':
    N = int(input())
    H = list(map(int, input().split()))
    print(solve(N, H))
