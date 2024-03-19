def solve(N, A) -> bool:
    now = (0, 0)
    D = []
    D.append(now)
    X = [[0 for _ in range(N)]for _ in range(N)]
    while D:
        now = D.pop()
        amount = A[now[0]][now[1]]
        if X[now[0]][now[1]]:
            continue
        X[now[0]][now[1]] = 1
        if amount == 0:
            continue
        elif amount == -1:
            return True
        if now[0] + amount >= N:
            pass
        else:
            target = (now[0] + amount, now[1])
            D.append(target)
        if now[1] + amount >= N:
            pass
        else:
            target = (now[0], now[1] + amount)
            D.append(target)

    return False


if __name__ == '__main__':
    N = int(input())
    A = [list(map(int, input().split())) for _ in range(N)]
    print("HaruHaru" if solve(N, A) else "Hing")
